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
    MakeProblemList {
        #[arg(short, long)]
        database_path: PathBuf,
        #[arg(short, long)]
        start: usize,
        #[arg(short, long)]
        end: usize,
        #[arg(short, long)]
        output: PathBuf,
    },
    CleanPackages {
        #[arg(short, long)]
        database_path: PathBuf,
        #[arg(short, long)]
        output_path: Option<PathBuf>,
    },
    MakeProblemSheet {
        #[arg(short, long)]
        input_path: PathBuf,
        #[arg(short, long)]
        problems_path: PathBuf,
        #[arg(short, long)]
        output_no_solutions: PathBuf,
        #[arg(short = 's', long)]
        output_with_solutions: PathBuf,
    },
}
