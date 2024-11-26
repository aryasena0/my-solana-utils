use std::fs::{self};

use clap::{Args, Parser, Subcommand};
use solana_sdk::signature::Keypair;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Keypair(KeypairArgs),
}

#[derive(Args)]
struct KeypairArgs {
    filepath: String,
    #[arg(short, long)]
    to_base_58: bool,
}

fn convert_keypair_to_base58(filepath: &str) {
    // Read the JSON keypair file
    let content = fs::read_to_string(filepath).expect("Failed to read keypair file");

    // Parse the JSON content into a Keypair
    let keypair: Vec<u8> = serde_json::from_str(&content).expect("Failed to parse keypair");
    let keypair = Keypair::from_bytes(&keypair);

    // Print the Base58 string representation of the keypair
    println!("{}", keypair.unwrap().to_base58_string());
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Keypair(args) => {
            let filepath = &args.filepath;
            convert_keypair_to_base58(filepath);
        }
    }
}
