mod args;
mod api;
mod download;
mod category;

use clap::Parser;
use log::{LevelFilter, info, debug};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    if args::Args::parse().debug {
        env_logger::builder().filter(None, LevelFilter::Debug).init();
        info!("Debug mode is enabled");
    }
    // Parse arguments
    let args = args::Args::parse();
    //Debug arguments
    debug!("Parsed arguments: {:#?}", args);

    let category = args.category;
    info!("Validating category: {}", category);
    let category = category::check(&category).await?;

    let file_name = args.name.unwrap();
    debug!("Filename from arguments: {}", file_name);

    // make from argument full file name
    let full_file_name = args.save_directory + "/" + &*file_name;
    debug!("Full file name is: {}", full_file_name);

    info!("Creating new reqwest client and sending a request for category: {}", category);
    let result = api::output(&category).await?;
    debug!("Received from HTTP response: {:#?}", result);

    println!("Artist: {}", result.artist_name);
    println!("Source image: {}", result.source_url);

    info!("Downloading image from {}", result.url);
    download::download_image(&result.url, &full_file_name).await?;

    Ok(())
}
