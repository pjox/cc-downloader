use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Download paths for a given snapshot
    DownloadPaths {
        /// Crawl reference
        #[arg(long, value_name = "SNAPSHOT")]
        snapshot: String,

        /// Data type
        #[arg(long, value_name = "PATHS")]
        data_type: String,

        /// Otput folder
        #[arg(short, long, value_name = "OUTPUT")]
        output: PathBuf,

        /// Print progress
        /// #[arg(short, long)]
        progress: Option<bool>,
    },

    /// Download files from a crawl
    Download {
        /// Path file
        #[arg(long, value_name = "PATHS")]
        path_file: PathBuf,

        /// Otput folder
        #[arg(short, long, value_name = "OUTPUT")]
        output: PathBuf,

        /// Print progress
        /// #[arg(short, long)]
        progress: Option<bool>,
    },
}
