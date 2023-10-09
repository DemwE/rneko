use serde::Deserialize;
use std::error::Error;
use colorful::{Color, Colorful};

#[derive(Clone, Deserialize)]
pub struct Neko {
    pub artist_href: String,
    pub artist_name: String,
    pub source_url: String,
    pub url: String,
}

#[derive(Deserialize)]
struct NekoResponse {
    results: Vec<Neko>,
}

pub fn nekos() -> Result<Neko, Box<dyn Error>> {
    let getinfo = reqwest::blocking::get("https://nekos.best/api/v2/neko")?;

    if getinfo.status().is_success() {
        let info = getinfo.text()?;
        // println!("{}", info);

        let parsed: NekoResponse = serde_json::from_str(&info)?;

        if let Some(neko) = parsed.results.first() {
            return Ok(neko.clone());
        } else {
            let error_message = "Error: No data in response".color(Color::Red);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message.to_string(),
            )));
        }

    } else {
        let error_message = format!("{}", getinfo.status()).color(Color::Red);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message.to_string(),
        )));
    }
}