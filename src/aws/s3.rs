use tokio::runtime::Runtime;

use crate::aws::config::make_client;

pub fn list_buckets() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut bucket_names = Vec::new();

    let s3 = make_client()?;
    let mut buckets = s3.list_buckets().into_paginator().send();
    let rt = Runtime::new()?;
    // 同期関数内で非同期関数を実行
    rt.block_on(async {
        // PaginationStream から逐次的にページを取得
        while let Some(page_result) = buckets.next().await {
            // ページ取得でエラーがあれば即座に返す
            let page = page_result;
            if let Ok(p) = page {
                // ページに含まれるバケットを処理
                if let Some(buckets) = p.buckets {
                    for bucket in buckets {
                        if let Some(name) = bucket.name {
                            bucket_names.push(name);
                        }
                    }
                }
            }
        }
    });

    Ok(bucket_names)
}
