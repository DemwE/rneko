mod args;
mod category;
mod api;
mod download;

use std::sync::Arc;
use clap::Parser;
use log::{LevelFilter, debug, error, info};
use download::download;
use futures::future::try_join_all;
use tokio::sync::Semaphore;
use indicatif::{ProgressBar, ProgressStyle};

#[tokio::main]
async fn main() {
    // Initialize the logger
    if args::Args::parse().debug {
        env_logger::builder().filter(None, LevelFilter::Debug).init();
        info!("Debug mode is enabled");
    }

    // Parse the arguments
    let args = args::Args::parse();

    // Check the category
    let category = args.category.to_lowercase();
    if !category::check(category.as_str()) {
        std::process::exit(1);
    }

    // Create a new progress bar if debug mode is disabled
    let pb = if !args.debug {
        let pb = ProgressBar::new(args.amount.into());
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}").unwrap()
            .progress_chars("#>-"));
        Some(pb)
    } else {
        info!("Debug mode is enabled - progress bar will not be shown");
        None
    };

    //Fetch api
    let results = api::get_images(category.as_str(), args.amount, args.debug).await;

    match results {
        Ok(results) => {
            info!("Fetched {} images", results.results.len());
            let mut tasks = Vec::new();
            let semaphore = Arc::new(Semaphore::new(args.workers));

            for (index, result) in results.results.iter().enumerate() {
                if let Some(pb) = &pb {
                    pb.inc(0);
                }
                let save_directory = args.save_directory.clone();
                let url = result.url.clone();
                let semaphore = Arc::clone(&semaphore);
                let index = if args.amount > 1 { Some(index as u16) } else { None };
                let name = args.name.clone();
                let pb = pb.clone();
                let task = tokio::spawn(async move {
                    let _permit = semaphore.acquire_owned().await;
                    let download_result = download(url, save_directory, &name, index).await;
                    if let Some(pb) = &pb {
                        pb.inc(1);
                    }
                    download_result
                });
                tasks.push(task);
            }
            // Wait for all tasks to complete
            let results = try_join_all(tasks).await;

            // Handle errors (if any)
            if let Err(e) = &results {
                error!("Failed to download some files: {:?}", e);
            }

            info!("Finished downloading images");
            info!("Downloaded {} images", results.unwrap().len());
            if !args.debug {
                pb.unwrap().finish_with_message("done");
            }
        }
        Err(error) => {
            debug!("Error: {}", error);
            std::process::exit(1);
        }
    }
}