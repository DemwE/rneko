mod args;
mod api;
mod download;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let args = args::Args::parse();

    let file_name = args.name.unwrap();

    // make from argument full file name
    let full_file_name = args.save_directory + "/" + &*file_name;

    let neko_result = api::nekos().await?;

    println!("Artist: {}", neko_result.artist_name);
    println!("Source image: {}", neko_result.source_url);

    download::download_image(&neko_result.url, &full_file_name).await?;

    Ok(())
}
