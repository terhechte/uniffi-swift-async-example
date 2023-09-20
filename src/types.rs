#[derive(uniffi::Record)]
pub struct Response {
    pub input: String,
    pub output: String,
    pub version: u8,
}
