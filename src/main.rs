// src/main.rs
/*
 * Main executable for BlockchainNFTVaultManager
 */

use clap::Parser;
use blockchainnftvaultmanager::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTVaultManager - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
