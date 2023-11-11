use clap::{command, Parser};

#[derive(Debug, Default, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(
help_template = "{name} {version} {author-section} {about-with-newline} \n {all-args}"
)]
pub struct Args {
    // Save directory for the file where default is current directory
    #[clap(default_value = ".")]
    #[clap(short, long)]
    pub save_directory: String,
    // Save file name when argument in -n or --name is used
    #[clap(default_value = "output.png")]
    #[clap(short, long)]
    pub name: Option<String>,
    /// Image category | neko, kitsune, waifu
    #[clap(default_value = "neko")]
    #[clap(short, long)]
    pub category: String,
    /// Activate debug mode
    #[clap(short, long)]
    pub debug: bool,
}