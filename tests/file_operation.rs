/// 1. ファイルの作成、書き込み、読み込み、削除の例
/// 2. ファイルへの追記（append）処理の例
/// 3. ファイルのリネーム（名前変更）の例
/// 4. ファイルの削除処理の例
/// 5. ディレクトリの作成と削除、およびディレクトリ内でのファイル操作の例
/// 6. 存在しないファイルのオープンによるエラー処理の例

#[cfg(test)]
mod tests {
    #[cfg(feature = "use_chrono")]
    use chrono::{DateTime, Local};
    use std::fs::{self, File, OpenOptions};
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// ユニークな一時ファイルのパスを生成するヘルパー関数
    fn get_temp_file_path(prefix: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        // 現在のUNIX時間（ナノ秒単位）を使ってファイル名にユニークな値を付与
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        path.push(format!("{}_{}.txt", prefix, nanos));
        path
    }

    /// 1. ファイルの作成、書き込み、読み込み、削除の例
    #[test]
    fn test_create_write_read_file() {
        let file_path = get_temp_file_path("test_file");
        {
            // ファイルを作成して内容を書き込む
            let mut file = File::create(&file_path).expect("ファイルの作成に失敗しました");
            writeln!(file, "Hello, file!").expect("ファイルへの書き込みに失敗しました");
        }

        // 作成したファイルを開いて内容を読み込む
        let mut contents = String::new();
        {
            let mut file = File::open(&file_path).expect("ファイルのオープンに失敗しました");
            file.read_to_string(&mut contents)
                .expect("ファイルの読み込みに失敗しました");
        }
        // writeln! は改行を入れるため、期待値は末尾に "\n" が含まれる
        assert_eq!(contents, "Hello, file!\n");

        // テスト終了後にファイルを削除
        fs::remove_file(&file_path).expect("ファイルの削除に失敗しました");
    }

    /// 2. ファイルへの追記（append）処理の例
    #[test]
    fn test_append_to_file() {
        let file_path = get_temp_file_path("test_append");
        {
            // 初回作成時はファイルに最初の行を書き込む
            let mut file = File::create(&file_path).expect("ファイルの作成に失敗しました");
            file.write_all(b"Line 1\n")
                .expect("ファイルへの書き込みに失敗しました");
        }
        {
            // OpenOptionsを使って追記モードでファイルをオープン
            let mut file = OpenOptions::new()
                .append(true)
                .open(&file_path)
                .expect("追記用にファイルをオープンできませんでした");
            file.write_all(b"Line 2\n").expect("追記に失敗しました");
        }
        // ファイル全体を読み込み、内容を検証
        let contents = fs::read_to_string(&file_path).expect("ファイルの読み込みに失敗しました");
        let expected = "Line 1\nLine 2\n";
        assert_eq!(contents, expected);

        // テスト終了後にファイルを削除
        fs::remove_file(&file_path).expect("ファイルの削除に失敗しました");
    }

    /// 3. ファイルのリネーム（名前変更）の例
    #[test]
    fn test_rename_file() {
        let file_path = get_temp_file_path("test_rename");
        // 新しいファイル名は一時ディレクトリ内の "renamed_file.txt" とする
        let mut new_file_path = std::env::temp_dir();
        new_file_path.push("renamed_file.txt");

        {
            // ファイルを作成して内容を書き込む
            let mut file = File::create(&file_path).expect("ファイルの作成に失敗しました");
            file.write_all(b"Content")
                .expect("ファイルへの書き込みに失敗しました");
        }

        // ファイルのリネーム（移動）を実施
        fs::rename(&file_path, &new_file_path).expect("ファイルのリネームに失敗しました");

        // 古いパスのファイルは存在せず、新しいパスに存在することを確認
        assert!(!file_path.exists());
        assert!(new_file_path.exists());

        // テスト終了後にリネーム後のファイルを削除
        fs::remove_file(&new_file_path).expect("ファイルの削除に失敗しました");
    }

    /// 4. ファイルの削除処理の例
    #[test]
    fn test_delete_file() {
        let file_path = get_temp_file_path("test_delete");
        {
            // ファイルを作成して内容を書き込む
            let mut file = File::create(&file_path).expect("ファイルの作成に失敗しました");
            file.write_all(b"Temporary content")
                .expect("ファイルへの書き込みに失敗しました");
        }
        // ファイルを削除する
        fs::remove_file(&file_path).expect("ファイルの削除に失敗しました");
        // ファイルが存在しないことを確認
        assert!(!file_path.exists());
    }

    /// 5. ディレクトリの作成と削除、およびディレクトリ内でのファイル操作の例
    #[test]
    fn test_create_and_remove_directory() {
        // 一時ディレクトリのパスを生成（ユニークな名前を付与）
        let mut dir_path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        dir_path.push(format!("test_dir_{}", nanos));

        // ディレクトリの作成
        fs::create_dir(&dir_path).expect("ディレクトリの作成に失敗しました");
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        // ディレクトリ内にファイルを作成
        let file_path = dir_path.join("inner_file.txt");
        {
            let mut file =
                File::create(&file_path).expect("ディレクトリ内のファイル作成に失敗しました");
            file.write_all(b"Hello inside dir")
                .expect("ディレクトリ内のファイル書き込みに失敗しました");
        }
        assert!(file_path.exists());

        // ファイルとディレクトリの削除
        fs::remove_file(&file_path).expect("内側のファイル削除に失敗しました");
        fs::remove_dir(&dir_path).expect("ディレクトリの削除に失敗しました");
        assert!(!dir_path.exists());
    }

    /// 6. 存在しないファイルのオープンによるエラー処理の例
    #[test]
    fn test_read_nonexistent_file() {
        let mut file_path = std::env::temp_dir();
        file_path.push("nonexistent_file.txt");

        let result = File::open(&file_path);
        // 存在しないファイルの場合はエラーが返るはず
        assert!(result.is_err());
    }

    /// 7. ファイルの更新日時取得
    #[test]
    #[cfg(feature = "use_chrono")]
    fn test_get_update_time() {
        // ファイルを作成
        let file_path = get_temp_file_path("test_file");
        {
            // ファイルを作成して内容を書き込む
            let mut file = File::create(&file_path).expect("ファイルの作成に失敗しました");
            writeln!(file, "Hello, file!").expect("ファイルへの書き込みに失敗しました");
        }

        let metadata = fs::metadata(file_path).unwrap();
        let modified_time = metadata.modified().unwrap();

        // SystemTime から chrono の DateTime に変換
        let datetime: DateTime<Local> = modified_time.into();
        println!("最終更新日時: {}", datetime.format("%Y-%m-%d %H:%M:%S"));
    }
}
