use anyhow::anyhow;
use log::{debug, info};

pub async fn download(url: String, save_directory: String, name: &String, index: Option<u16>) -> Result<(), anyhow::Error> {
    // Create the save path
    let extension = url.split('.').last().unwrap().to_string();
    let save_path = match index {
        Some(i) => format!("{}/{}-{}.{}", save_directory, name, i + 1, extension),
        None => format!("{}/{}.{}", save_directory, name, extension),
    };
    let save_name = save_path.split('/').last().unwrap().to_string();

    debug!("Save path: {}", save_path);

    let max_retries = 3;
    let mut retries = 0;

    while retries < max_retries {
        // Fetch the image
        let response = reqwest::get(url.as_str()).await;
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Successfully fetched image: {}", save_name);
                    // Save the image
                    let bytes = response.bytes().await?;
                    tokio::fs::write(save_path, bytes).await?;
                    info!("Successfully saved image: {} in path: {}", save_name, save_directory);
                    break;
                } else {
                    return Err(anyhow!("Failed to fetch image"));
                }
            }
            Err(e) => {
                retries += 1;
                debug!("Failed to fetch image on attempt {}: {}", retries, e);
                if retries >= max_retries {
                    return Err(anyhow!("Failed to fetch image after {} attempts", max_retries));
                }
            }
        }
    }

    Ok(())
}