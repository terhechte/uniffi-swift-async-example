use async_std::future::{pending, timeout};
use std::time::Duration;

pub mod types;
use types::Response;

#[uniffi::export]
pub fn transform(input: String) -> Option<String> {
    andaluh::epa(&input).ok()
}

#[uniffi::export]
pub async fn expensive(input: String) -> Option<Response> {
    let never = pending::<()>();
    timeout(Duration::from_millis(1000), never)
        .await
        .unwrap_err();
    andaluh::epa(&input).ok().map(|output| Response {
        input,
        output,
        version: 42,
    })
}

uniffi::setup_scaffolding!();
