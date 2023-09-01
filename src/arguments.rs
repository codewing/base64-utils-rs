use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct ProgramArguments {
    #[command(subcommand)]
    pub command: Option<ProgramMode>,
}

#[derive(Subcommand)]
pub enum ProgramMode {
    Encode {
        #[arg(short, long)]
        file_path: PathBuf,
    },

    Decode {
        #[arg(short, long)]
        text: String,
    },
}