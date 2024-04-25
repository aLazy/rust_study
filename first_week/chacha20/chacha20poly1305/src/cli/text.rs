use super::{verify_file, verify_path};
use clap::Parser;
use std::{fmt, path::PathBuf, str::FromStr};
#[derive(Parser, Debug)]
pub enum TextSubCommand {
    #[command(name = "encrypt", about = "Encrypt text")]
    Encrypt(EncryptOpt),
    #[command(name = "decrypt", about = "Decrypt text")]
    Decrypt(DecryptOpt),
    #[command(name = "generate", about = "Generate text")]
    Generate(GenerateOpt),
}
#[derive(Parser, Debug)]
pub struct EncryptOpt {
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, default_value = "-", value_parser=verify_file)]
    pub plaintext: String,
    #[arg(short, long, value_parser = verify_file)]
    pub nonce: String,
    #[arg(short, long, default_value = "chacha20-poly1305")]
    pub format: TextsigFormat,
}

#[derive(Parser, Debug)]
pub struct DecryptOpt {
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, default_value = "-",value_parser = verify_file)]
    pub ciphertext: String,
    #[arg(short, long, value_parser = verify_file)]
    pub nonce: String,
    #[arg(short, long, default_value = "chacha20-poly1305")]
    pub format: TextsigFormat,
}

#[derive(Parser, Debug)]
pub struct GenerateOpt {
    #[arg(short,long,value_parser = verify_path)]
    pub key_path: PathBuf,
    #[arg(short, long, default_value = "chacha20-poly1305")]
    pub format: TextsigFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextsigFormat {
    Chacha20Poly1305,
}

impl FromStr for TextsigFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        match s {
            "chacha20-poly1305" => Ok(TextsigFormat::Chacha20Poly1305),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
impl From<TextsigFormat> for &'static str {
    fn from(format: TextsigFormat) -> Self {
        match format {
            TextsigFormat::Chacha20Poly1305 => "chacha20-poly1305",
        }
    }
}

impl fmt::Display for TextsigFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
