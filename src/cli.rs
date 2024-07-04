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
        /// Crawl reference, e.g. CC-MAIN-2021-04
        #[arg(value_name = "SNAPSHOT")]
        snapshot: String,

        /// Data type
        #[arg(value_name = "PATHS")]
        data_type: String,

        /// Destination folder
        #[arg(value_name = "DESTINATION")]
        dst: PathBuf,

        /// Print progress
        progress: Option<bool>,
    },

    /// Download files from a crawl
    Download {
        /// Path file
        #[arg(value_name = "PATHS")]
        path_file: PathBuf,

        /// Destination folder
        #[arg(value_name = "DESTINATION")]
        dst: PathBuf,

        /// Enumerate output files for compatibility with Ungoliant Pipeline
        #[arg(short, long)]
        numbered: bool,

        /// Print progress
        progress: Option<bool>,
    },
}
