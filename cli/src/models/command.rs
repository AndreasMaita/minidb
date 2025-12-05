use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "minidb")]
pub struct CLI {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    Load {
        path: std::path::PathBuf,
    },
    Save {
        path: std::path::PathBuf,
    },
    Dump {
        path: std::path::PathBuf,
    },
    Create {
        #[command(subcommand)]
        subcommand: CreateCommand,
    },
}

#[derive(Subcommand, Clone)]
pub enum CreateCommand {
    Table { name: String, database: String },
    Database { name: String },
}
