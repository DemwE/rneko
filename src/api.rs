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

pub async fn nekos() -> Result<Neko, Box<dyn Error>> {
    let getinfo = reqwest::get("https://nekos.best/api/v2/neko").await?;

    let error_message = if getinfo.status().is_success() {
        let info = getinfo.text().await?;

        let parsed: NekoResponse = serde_json::from_str(&info)?;

        if let Some(neko_result) = parsed.results.first() {
            return Ok(neko_result.clone());
        }

        "Error: No data in response".to_string().color(Color::Red).to_string()
    } else {
        format!("HTTP error: {}", getinfo.status())
    };

    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
}