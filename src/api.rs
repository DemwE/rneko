use serde::Deserialize;
use std::error::Error;
use colorful::{Color, Colorful};
use log::{debug, error, info};

#[derive(Clone, Deserialize, Debug)]
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

pub async fn output(category: &String) -> Result<Neko, Box<dyn Error>> {
    debug!("Fetching data for category: {}", category);
    let url = format!("https://nekos.best/api/v2/{}", category);
    debug!("URL to fetch data: {}", url);

    let getinfo = reqwest::get(&url).await?;
    debug!("Response received from the server");

    if !getinfo.status().is_success() {
        error!("HTTP error: {}", getinfo.status());
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("HTTP error: {}", getinfo.status()))))
    } else {
        debug!("HTTP request was successful");
        let info = getinfo.text().await?;
        info!("Response body received");
        debug!("Response body: {}", info);

        let parsed: NekoResponse = serde_json::from_str(&info)?;

        if let Some(neko_result) = parsed.results.first() {
            info!("Neko result obtained");
            debug!("Neko result: {:?}", neko_result);
            Ok(neko_result.clone())
        } else {
            error!("No data in the response");
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error: No data in response".to_string().color(Color::Red).to_string())))
        }
    }
}