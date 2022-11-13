use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct RawArgs {
    #[command(subcommand)]
    command: Action,
    #[arg(short, long)]
    output_dir: PathBuf,

    #[arg(short, long)]
    problems_dir: PathBuf,

    #[arg(short, long)]
    database_dir: Option<PathBuf>,
}

#[derive(Debug)]
pub struct MyArgs {
    pub command: Action,
    pub output_dir: PathBuf,
    pub problems_dir: PathBuf,
    pub database_dir: PathBuf,
}

impl MyArgs {
    pub fn get() -> Self {
        let args = RawArgs::parse();

        Self {
            output_dir: args.output_dir,
            database_dir: args
                .database_dir
                .unwrap_or_else(|| args.problems_dir.join("database.json")),
            problems_dir: args.problems_dir,
            command: args.command,
        }
    }
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    SyncDb,
}
