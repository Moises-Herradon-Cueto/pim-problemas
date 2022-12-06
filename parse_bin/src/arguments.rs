use std::path::PathBuf;

use clap::{Parser, Subcommand};
use parse_lib::Fields;

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
        #[arg(long)]
        database_dir_out: Option<PathBuf>,
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
    MakeHtml {
        #[arg(short, long)]
        database_path: PathBuf,
        #[arg(short, long)]
        output_path: PathBuf,
    },
    Regex {
        #[arg(short = 's', long)]
        regex: String,
        #[arg(short, long)]
        replacement: String,
        #[arg(value_enum, short, long)]
        field: Option<Fields>,
        #[arg(short, long)]
        database_path: PathBuf,
        #[arg(short, long)]
        output_path: Option<PathBuf>,
    },
    RegexFromFile {
        #[arg(short, long)]
        regex_file: PathBuf,
        #[arg(short, long)]
        database_path: PathBuf,
        #[arg(short, long)]
        output_path: Option<PathBuf>,
    },
    GetTopics {
        #[arg(short, long)]
        database_path: PathBuf,
        #[arg(long)]
        php: bool,
    },
    Sql {
        #[arg(short, long)]
        database_path: PathBuf,
    },
}
