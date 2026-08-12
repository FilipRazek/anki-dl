use anyhow::Result;
use serde::Deserialize;
use std::io::Read;
use zstd::Decoder;

#[derive(Debug, Deserialize)]
pub struct LoginResult {
    pub key: String,
}

impl LoginResult {
    pub fn decompress_result<R: Read>(bytes: R) -> Result<LoginResult> {
        let decoder = Decoder::new(bytes)?;
        Ok(serde_json::from_reader(decoder)?)
    }
}
