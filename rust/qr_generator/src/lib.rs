use anyhow;
mod helpers;
mod metadata_shortcut;
    use metadata_shortcut::meta_shortcut;
use qrcode_rtx::make_pretty_qr;
mod error;
pub mod config;

use crate::error::Error;

pub fn generate_metadata_qr(chain: &config::Chain) -> anyhow::Result<()> {
    println!("Fetching metadata for {} from {}", chain.name, chain.url);
    let shortcut = meta_shortcut(&chain.url)?;

    let crypto_type_code = "ff";
    let prelude = format!("53{}{}", crypto_type_code, "80");
    let complete_message = [hex::decode(prelude).expect("known value"), shortcut.meta_values.meta].concat();
    let output_name= format!("unsigned_qr/{}", chain.name);
    println!("generating QR for {}. It takes a while...", chain.name);
    if let Err(e) = make_pretty_qr(&complete_message, &output_name) {
        return Err(Error::Qr(e.to_string()).show())
    }
    Ok(())
}
