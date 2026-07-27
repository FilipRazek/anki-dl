use std::io::{Error, Read};
use serde::Deserialize;
use zstd::Decoder;

#[derive(Debug, Deserialize)]
pub struct LoginResult {
    pub key: String,
} 

pub fn decompress_result<R: Read>(bytes: R) -> Result<LoginResult, Error> {
    let decoder = Decoder::new(bytes)?;
    Ok(serde_json::from_reader(decoder)?)
}