use async_std::future::{pending, timeout};
use std::time::Duration;

#[uniffi::export]
pub fn transform(input: String) -> Option<String> {
    andaluh::epa(&input).ok()
}

#[uniffi::export]
pub async fn expensive(input: String) -> Option<String> {
    let never = pending::<()>();
    timeout(Duration::from_millis(1000), never)
        .await
        .unwrap_err();
    andaluh::epa(&input).ok()
}

uniffi::setup_scaffolding!();
