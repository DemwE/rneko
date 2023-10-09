mod args;
mod api;
mod download;

use clap::Parser;
use std::error::Error;
use colorful::{Color, Colorful};

fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments
    let args = args::Args::parse();

    let file_name = args.name.unwrap();

    // make from argument full file name
    let full_file_name = args.save_directory + "/" + &*file_name;

    let neko_result = api::nekos();

    match &neko_result {
        Ok(neko) => {
            let image = neko.url.clone();
            let source_image = neko.source_url.clone();
            let artist = neko.artist_name.clone();

            // println!("Image: {}", image);
            println!("Artist: {}", artist);
            println!("Source image: {}", source_image);

            download::download_image(&image, &full_file_name)?;
        }
        Err(err) => {
            let error_message = format!("Error: {}", err);
            eprintln!("{}", error_message.color(Color::Red));
        }
    };

    Ok(())
}
