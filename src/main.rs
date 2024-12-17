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
        }) => {
            download::download_paths(snapshot, data_type.as_str(), dst)
                .await
                .expect("Error downloading paths");
        }
        Some(Commands::Download {
            path_file,
            dst,
            progress,
            threads,
            retries,
            numbered,
        }) => {
            download::download(path_file, dst, *threads, *retries, *numbered, *progress)
                .await
                .expect("Error downloading files");
        }
        None => {
            println!("No command specified");
        }
    }
}
