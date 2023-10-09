use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use colorful::{Color, Colorful};

pub fn download_image(url: &str, save_path: &str) -> Result<(), Box<dyn Error>> {
    // Parse URL
    let url = Url::parse(url)?;

    // Make GET request
    let mut response = reqwest::blocking::get(url)?;

    if response.status().is_success() {
        // Get total file size from response headers
        let total_size = response.content_length().unwrap_or(0);
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} | {binary_bytes_per_sec} | eta {eta} ")
                .unwrap()
                .progress_chars("#>-"),
        );

        // Open file for writing
        let file = File::create(save_path)?;
        let mut buffered_file = BufWriter::new(file);

        // Read response in chunks and write to file with progress update
        let mut buffer = [0; 65536]; // Buffer size of 64KB
        let mut downloaded = 0;
        loop {
            let bytes_read = response.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            buffered_file.write_all(&buffer[..bytes_read])?;
            downloaded += bytes_read;
            pb.set_position(downloaded as u64);
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


