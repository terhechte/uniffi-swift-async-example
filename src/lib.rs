use async_std::future::{pending, timeout};
use std::{sync::Arc, time::Duration};

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

#[derive(uniffi::Object)]
pub struct TicketHandler {}

#[uniffi::export]
impl TicketHandler {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(TicketHandler {})
    }

    pub async fn get_tickets(&self) -> Vec<String> {
        Vec::new()
    }
}

uniffi::setup_scaffolding!();
