use clap::Parser;

use crate::cli::Commands;

mod cli;
mod download;
mod errors;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        Some(Commands::DownloadPaths {
            snapshot,
            data_type,
            dst,
            progress: _,
        }) => {
            download::download_paths(snapshot, data_type.as_str(), dst)
                .await
                .expect("Error downloading paths");
            println!("Downloading paths: ",);
        }
        Some(Commands::Download {
            path_file,
            dst,
            progress: _,
            threads,
            numbered,
        }) => {
            download::download(path_file, dst, *threads, numbered)
                .await
                .expect("Error downloading files");
            println!("Downloading paths: ",);
        }
        None => {
            println!("No command specified");
        }
    }
}
