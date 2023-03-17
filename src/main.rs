mod cli;

use clap::Parser;
use cli::Cli;
use std::process;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.run().await {
        Ok(str) => {
            if let Some(s) = str {
                println!("{s}");
            }
        }
        Err(err) => {
            eprintln!("{err:?}");
            process::exit(1);
        }
    }
}
