mod text;
pub use self::text::*;

use clap::Parser;
use std::path::{Path, PathBuf};
#[derive(Debug, Parser)]
#[command(name = "rcli", author , version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, Parser)]
pub enum Subcommands {
    #[command(subcommand)]
    Text(TextSubCommand),
}

pub fn verify_path(s: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(s);
    if path.exists() && path.is_dir() {
        Ok(path.into())
    } else {
        Err("Invalid path")
    }
}

pub fn verify_file(file: &str) -> Result<String, &'static str> {
    let path = Path::new(file);
    if path.exists() || file == "-" {
        Ok(file.into())
    } else {
        Err("Invalid file")
    }
}
