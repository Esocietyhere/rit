mod cli;
mod color;
mod config;
mod rbx;

use clap::Parser;
use cli::Cli;
use dotenv::dotenv;
use std::process;

#[tokio::main]
async fn main() {
    dotenv().ok();
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
