use crate::infrastructure::config;
use clap::{Parser, Subcommand};
use mockall::*;

use crate::adapters::cmd::subcmd::SubCmd;
use crate::domain::error::AppError;
use core::result::Result;

#[derive(Parser, Debug, PartialEq)]
#[command(name = "task-cli", about = "A simple task management CLI application")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    List { status: Option<String> },
    Add { description: String },
    Update { id: u32, description: String },
    Delete { id: u32 },
    MarkInProgress { id: u32 },
    MarkDone { id: u32 },
}

// TDD
// ✅ ❔ ❌
// 3.1. buatlah struktur data Command
// => 3.1. create the Command Data Structure
// ------------------------------------------------
#[derive(Clone)]
pub struct Command {}

// TDD
// ✅ ❔ ❌
// 3.2.
// => the CommandTrait trait for the Command struct
// ------------------------------------------------
#[automock]
pub trait CommandTrait {
    fn run(config: &config::Config) -> Result<String, AppError>;
}

impl CommandTrait for Command {
    fn run(config: &config::Config) -> Result<String, AppError> {
        let cli = Cli::parse();
        let subcmd = SubCmd::new(config);
        match &cli.commands {
            /*
                List Task Operations
                -------------------------------
                $ task-cli list
                $ task-cli list [todo|in-progress|done]
            */
            Commands::List { status } => subcmd.lists(status),
            /*
                Task Operations
                -------------------------------
                $ task-cli add <description>
                $ task-cli update <id> <description>
                $ task-cli delete <id>
            */
            Commands::Add { description } => subcmd.add(description),
            Commands::Update { id, description } => subcmd.update(id, description),
            Commands::Delete { id } => subcmd.delete(id),
            /*
                Mark Task Operations
                -------------------------------
                $ task-cli [mark-in-progress|mark-done] <id>
            */
            Commands::MarkInProgress { id } => subcmd.mark_in_progress(id),
            Commands::MarkDone { id } => subcmd.mark_done(id),
        }
    }
}
