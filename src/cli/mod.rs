mod base64;
mod csv;
mod genpass;
mod http;
mod text;
use clap::Parser;
use std::path::{Path, PathBuf};

use crate::CmdExector;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
    http::HttpSubCommand,
    text::{TextSignFormat, TextSubCommand},
};
use self::{csv::CsvOpts, genpass::GenPassOpts};

#[derive(Debug, Parser)]
#[command(name = "rcli", version = "0.1.0", author = "Rust CLI", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode or decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Sign or verify text")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

impl CmdExector for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await?,
            SubCommand::GenPass(opts) => opts.execute().await?,
            SubCommand::Base64(cmd) => cmd.execute().await?,
            SubCommand::Text(cmd) => cmd.execute().await?,
            SubCommand::Http(cmd) => cmd.execute().await?,
        }
        Ok(())
    }
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}
