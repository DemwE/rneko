use reqwest::Url;
use std::error::Error;
use std::fs::File;
use std::io::{Write, BufWriter};
use indicatif::{ProgressBar, ProgressStyle};
use colorful::{Color, Colorful};

pub async fn download_image(url: &str, save_path: &str) -> Result<(), Box<dyn Error>> {
    // Parse URL
    let url = Url::parse(url)?;

    // Make GET request
    let mut response = reqwest::get(url).await?;

    if response.status().is_success() {
        // Get total file size from response headers
        let total_size = response.content_length().unwrap_or(0);
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} | {binary_bytes_per_sec} | eta {eta}")
                .unwrap()
                .progress_chars("#>-"),
        );

        // Open file for writing
        let file = File::create(save_path)?;
        let mut buffered_file = BufWriter::new(file);

        // Read response in chunks and write to file with progress update
        let mut downloaded = 0;
        while let Some(chunk) = response.chunk().await? {
            buffered_file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }

        buffered_file.flush()?; // Flush the buffer to ensure all data is written to disk

        Ok(())
    } else {
        let error_message = format!("Error: {}", response.status());
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message.color(Color::Red).to_string(),
        )))
    }
}
