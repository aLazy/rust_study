use std::fs;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use clap::Parser;
use rcli::{
    get_content, get_input, process_decrypt_fun, process_encrypt_func, process_key_generate, Cli,
    Subcommands, TextSubCommand,
};
fn main() -> anyhow::Result<()> {
    let opts = Cli::parse();
    match opts.cmd {
        Subcommands::Text(subcommand) => match subcommand {
            TextSubCommand::Encrypt(opts) => {
                let key = get_content(&opts.key)?;
                let nonce = get_content(&opts.nonce)?;
                let mut text = get_input(&opts.plaintext)?;
                let sig = process_encrypt_func(&key, &nonce, &mut text)?;
                let encoded = STANDARD.encode(&sig);
                println!("{:?}", encoded);
            }
            TextSubCommand::Decrypt(opts) => {
                let key = get_content(&opts.key)?;
                let nonce = get_content(&opts.nonce)?;
                let mut text = get_input(&opts.ciphertext)?;
                let mut tmp = String::new();
                text.read_to_string(&mut tmp).unwrap();
                let decoded = STANDARD.decode(tmp)?;
                let verify = process_decrypt_fun(&key, &nonce, decoded.as_slice())?;

                print!("{:?}", String::from_utf8(verify.clone()).unwrap());
            }
            TextSubCommand::Generate(opts) => {
                let map = process_key_generate(opts.format)?;
                for (k, v) in map {
                    fs::write(opts.key_path.join(k), v).unwrap();
                }
            }
        },
    }
    Ok(())
}
