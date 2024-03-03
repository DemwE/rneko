use log::{debug, info};
use serde::Deserialize;
use reqwest::Error;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Deserialize)]
pub struct Result {
    pub artist_href: String,
    pub artist_name: String,
    pub source_url: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Results {
    pub results: Vec<Result>,
}

pub async fn get_images(category: &str, amount: u16, debug: bool) -> std::result::Result<Results, Error> {
    let api_limit = 20;
    let full_requests = amount / api_limit;
    let remainder = amount % api_limit;
    debug!("Full requests: {}", full_requests);
    debug!("Remainder: {}", remainder);

    let mut all_results = Vec::new();

    // Create a new progress bar if debug mode is disabled
    let pb = if !debug {
        let pb = ProgressBar::new((full_requests + if remainder > 0 { 1 } else { 0 }).into());
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}").unwrap()
            .progress_chars("#>-"));
        Some(pb)
    } else {
        info!("Debug mode is enabled - progress bar will not be shown");
        None
    };

    if let Some(pb) = &pb {
        pb.inc(0);
    }
    
    for _ in 0..full_requests {
        let url = format!("https://nekos.best/api/v2/{}?amount={}", category, api_limit);
        info!("Fetching images from: {}", url);
        let response = reqwest::get(&url).await?;
        let results = response.json::<Results>().await?;
        all_results.extend(results.results);
        if let Some(pb) = &pb {
            pb.inc(1);
        }
    }

    if remainder > 0 {
        let url = format!("https://nekos.best/api/v2/{}?amount={}", category, remainder);
        info!("Fetching images from: {}", url);
        let response = reqwest::get(&url).await?;
        let results = response.json::<Results>().await?;
        all_results.extend(results.results);
        if let Some(pb) = &pb {
            pb.inc(1);
        }
    }

    if !debug {
        pb.unwrap().finish_with_message("done");
    }
    Ok(Results { results: all_results })
}