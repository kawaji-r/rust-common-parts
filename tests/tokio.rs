//! tokioを使った非同期処理の使い方を学びます

#![cfg(feature = "use_tokio")]

use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

/// 同期関数内から非同期処理を実行する方法
#[test]
fn test_sync_function() {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
        sleep(Duration::from_millis(500)).await;
        42
    });
    assert_eq!(result, 42);
}
