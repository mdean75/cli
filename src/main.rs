mod matches;

use clap::Parser;
use anyhow::{Context, Result};
use matches::find_matches;
use std::env;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version)]
#[clap(about, long_about = None)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[arg()]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {

    // println!("Build TimeStamp: {}", env!("BUILD_INFO"));
    //
    // let version = option_env!("PROJECT_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    // println!("This binary was built from {}", version);

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", args.path))?;


    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
