mod args;
mod api;
mod download;
mod category;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let args = args::Args::parse();
    let category = args.category;
    let category = category::check(&category).await?;

    let file_name = args.name.unwrap();

    // make from argument full file name
    let full_file_name = args.save_directory + "/" + &*file_name;

    let result = api::output(&category).await?;

    println!("Artist: {}", result.artist_name);
    println!("Source image: {}", result.source_url);

    download::download_image(&result.url, &full_file_name).await?;

    Ok(())
}
