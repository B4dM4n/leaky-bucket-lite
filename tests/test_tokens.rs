use leaky_bucket_lite::{sync_threadsafe::LeakyBucket as SyncLeakyBucket, LeakyBucket};
use tokio::time::sleep;

use std::time::Duration;

#[tokio::test]
async fn test_tokens() {
    let rate_limiter = LeakyBucket::builder()
        .max(5)
        .tokens(5)
        .refill_amount(1)
        .refill_interval(Duration::from_millis(100))
        .build();

    assert_eq!(rate_limiter.tokens(), 5);

    for i in 0..5 {
        assert_eq!(rate_limiter.tokens(), 5 - i);
        rate_limiter.acquire_one().await;
    }

    assert_eq!(rate_limiter.tokens(), 0);
    rate_limiter.acquire_one().await;
    assert_eq!(rate_limiter.tokens(), 0);
}

#[test]
fn test_tokens_sync_threadsafe() {
    let rate_limiter = SyncLeakyBucket::builder()
        .max(5)
        .tokens(5)
        .refill_amount(1)
        .refill_interval(Duration::from_millis(100))
        .build();

    assert_eq!(rate_limiter.tokens(), 5);

    for i in 0..5 {
        assert_eq!(rate_limiter.tokens(), 5 - i);
        rate_limiter.acquire_one();
    }

    assert_eq!(rate_limiter.tokens(), 0);
    rate_limiter.acquire_one();
    assert_eq!(rate_limiter.tokens(), 0);
}

#[tokio::test]
async fn test_concurrent_tokens() {
    let rate_limiter = LeakyBucket::builder()
        .max(5)
        .tokens(0)
        .refill_amount(1)
        .refill_interval(Duration::from_millis(100))
        .build();

    let rl = rate_limiter.clone();
    tokio::spawn(async move {
        rl.acquire(5).await;
    });

    for i in 0..5 {
        assert_eq!(rate_limiter.tokens(), i);
        sleep(Duration::from_millis(100)).await;
    }

    assert_eq!(rate_limiter.tokens(), 0);
}
