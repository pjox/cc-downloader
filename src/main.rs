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
            match download::download_paths(snapshot, data_type.as_str(), dst).await {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error downloading paths: {}", e);
                }
            };
        }
        Some(Commands::Download {
            path_file,
            dst,
            progress,
            threads,
            retries,
            numbered,
            files_only,
        }) => {
            if *numbered && *files_only {
                eprintln!("Numbered and Files Only flags are incompatible");
            } else {
                match download::download(
                    path_file,
                    dst,
                    *threads,
                    *retries,
                    *numbered,
                    *files_only,
                    *progress,
                )
                .await
                {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Error downloading paths: {}", e);
                    }
                };
            }
        }
        None => {
            eprintln!("No command specified");
        }
    }
}
