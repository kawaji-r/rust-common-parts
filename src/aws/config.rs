use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_s3::config::Credentials as S3Credentials;
use aws_sdk_sts::{Client as StsClient, Error as StsError};
use aws_types::region::Region;
use dotenv::dotenv;
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fs;
use std::io::{self, Write};
use std::time::SystemTime;
use tokio::runtime::Runtime;

/// キャッシュされた認証情報を保持する構造体
#[derive(Serialize, Deserialize)]
struct CachedCredentials {
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
    expiration: SystemTime,
}

/// 同期関数として S3 クライアントを生成し、バケット一覧を表示する  
pub fn make_client() -> Result<aws_sdk_s3::Client, Box<dyn std::error::Error>> {
    // Tokio ランタイムの生成
    let rt = Runtime::new()?;
    rt.block_on(async {
        // .env ファイルから環境変数を読み込む（存在すれば）
        dotenv().ok();

        // AWS_REGION は必須
        let region_str =
            std::env::var("AWS_REGION").expect("AWS_REGION 環境変数が設定されていません");
        let region = Region::new(region_str);

        // S3 クライアントを保持する変数（後で生成）
        let s3_client;

        // ROLE_ARN が設定されている場合は AssumeRole を実行する
        if std::env::var("ROLE_ARN").is_ok() {
            // AssumeRole の認証情報はキャッシュから読み出す（有効期限内なら再利用）
            let creds = if let Some(cached) = load_cached_credentials() {
                println!(
                    "キャッシュ済みの認証情報を利用します（有効期限: {:?}）",
                    cached.expiration
                );
                cached
            } else {
                // キャッシュがない／期限切れの場合、MFA_SERIAL の有無により処理を分岐
                let new_creds = if std::env::var("MFA_SERIAL").is_ok() {
                    // MFA_SERIAL が設定されている場合は MFA 認証を実行
                    print!("MFAコードを入力してください: ");
                    io::stdout().flush()?;
                    let mfa_code = read_password()?.trim().to_string();
                    assume_role_with_mfa(&mfa_code).await?
                } else {
                    // MFA_SERIAL がなければ MFA を使わず AssumeRole を実行
                    assume_role_without_mfa().await?
                };
                // 取得した認証情報をキャッシュする
                cache_credentials(&new_creds);
                new_creds
            };

            println!("Assumed Role の認証情報:");
            println!("  Access Key: {}", creds.access_key_id);
            println!("  Secret Key: {}", creds.secret_access_key);
            println!("  Session Token: {}", creds.session_token);
            println!("  有効期限 (UNIX Timestamp): {:?}", creds.expiration);

            // AssumeRole の認証情報から S3 用の認証情報プロバイダーを作成
            let s3_credentials = S3Credentials::new(
                creds.access_key_id,
                creds.secret_access_key,
                Some(creds.session_token),
                None,          // 有効期限は SDK 内で管理するため None
                "assume_role", // 認証情報のソース名（任意）
            );

            // SDK のデフォルト設定をロードし、上書きして S3 クライアントの設定を作成
            let config = load_defaults(BehaviorVersion::latest()).await;
            let s3_config = aws_sdk_s3::config::Builder::from(&config)
                .credentials_provider(s3_credentials)
                .region(Some(region))
                .build();
            s3_client = aws_sdk_s3::Client::from_conf(s3_config);
        } else {
            // ROLE_ARN が設定されていない場合は、AssumeRole を実行せずデフォルト認証情報を利用
            let config = load_defaults(BehaviorVersion::latest()).await;
            let s3_config = aws_sdk_s3::config::Builder::from(&config)
                .region(Some(region))
                .build();
            s3_client = aws_sdk_s3::Client::from_conf(s3_config);
        }

        Ok(s3_client)
    })
}

/// MFA コードを利用して STS の AssumeRole を実行する関数  
/// ※ `mfa_token`: ユーザーが入力した MFA のトークンコード
async fn assume_role_with_mfa(mfa_token: &str) -> Result<CachedCredentials, StsError> {
    // SDK のデフォルト設定をロード
    let config = load_defaults(BehaviorVersion::latest()).await;
    let sts_client = StsClient::new(&config);

    // ROLE_ARN と MFA_SERIAL は環境変数から取得
    let role_arn = std::env::var("ROLE_ARN").expect("ROLE_ARN 環境変数が設定されていません");
    let mfa_serial = std::env::var("MFA_SERIAL").expect("MFA_SERIAL 環境変数が設定されていません");
    let role_session_name = "my-session";

    // AssumeRole リクエストを作成し、MFA 認証情報を渡して実行
    let resp = sts_client
        .assume_role()
        .role_arn(&role_arn)
        .role_session_name(role_session_name)
        .duration_seconds(3600) // 1 時間の有効期限
        .serial_number(&mfa_serial)
        .token_code(mfa_token)
        .send()
        .await?;

    let creds = resp.credentials().expect("認証情報が返されていません");
    let exp_system_time = SystemTime::try_from(creds.expiration)
        .expect("認証情報の有効期限を SystemTime に変換できませんでした");

    Ok(CachedCredentials {
        access_key_id: creds.access_key_id().to_string(),
        secret_access_key: creds.secret_access_key().to_string(),
        session_token: creds.session_token().to_string(),
        expiration: exp_system_time,
    })
}

/// MFA 認証を行わずに AssumeRole を実行する関数
async fn assume_role_without_mfa() -> Result<CachedCredentials, StsError> {
    // SDK のデフォルト設定をロード
    let config = load_defaults(BehaviorVersion::latest()).await;
    let sts_client = StsClient::new(&config);

    // ROLE_ARN は環境変数から取得
    let role_arn = std::env::var("ROLE_ARN").expect("ROLE_ARN 環境変数が設定されていません");
    let role_session_name = "my-session";

    // AssumeRole リクエストを作成（MFA を使わない）
    let resp = sts_client
        .assume_role()
        .role_arn(&role_arn)
        .role_session_name(role_session_name)
        .duration_seconds(3600) // 1 時間の有効期限
        .send()
        .await?;

    let creds = resp.credentials().expect("認証情報が返されていません");
    let exp_system_time = SystemTime::try_from(creds.expiration)
        .expect("認証情報の有効期限を SystemTime に変換できませんでした");

    Ok(CachedCredentials {
        access_key_id: creds.access_key_id().to_string(),
        secret_access_key: creds.secret_access_key().to_string(),
        session_token: creds.session_token().to_string(),
        expiration: exp_system_time,
    })
}

/// キャッシュファイル（JSON形式）から認証情報を読み込む関数  
/// ※ キャッシュが存在し、かつ有効期限が現在よりも先なら Some を返す
fn load_cached_credentials() -> Option<CachedCredentials> {
    let cache_file = "cached_credentials.json";
    if let Ok(data) = fs::read_to_string(cache_file) {
        if let Ok(creds) = serde_json::from_str::<CachedCredentials>(&data) {
            if creds.expiration > SystemTime::now() {
                return Some(creds);
            }
        }
    }
    None
}

/// 取得した認証情報をキャッシュファイル（JSON形式）に書き出す関数  
/// ※ 書き込みエラーは無視する
fn cache_credentials(creds: &CachedCredentials) {
    let cache_file = "cached_credentials.json";
    if let Ok(json) = serde_json::to_string(creds) {
        let _ = fs::write(cache_file, json);
    }
}
