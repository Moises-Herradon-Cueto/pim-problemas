use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct MyArgs {
    #[command(subcommand)]
    pub command: Action,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    SyncDb {
        #[arg(short, long)]
        output_dir: PathBuf,
        #[arg(short, long)]
        problems_dir: PathBuf,
        #[arg(short, long)]
        database_dir: Option<PathBuf>,
    },
    CompareCsvJson {
        #[arg(short, long)]
        merged_path: PathBuf,
        #[arg(short, long)]
        database_dir: PathBuf,
    },
    WriteCsv {
        #[arg(short, long)]
        csv_dir: PathBuf,
        #[arg(short, long)]
        database_dir: PathBuf,
    },
    Latex {
        #[arg(short, long)]
        output_dir: PathBuf,
    },
    Migrate {
        #[arg(short, long)]
        database_dir: PathBuf,
        #[arg(short, long)]
        new_database_dir: PathBuf,
    },
}
