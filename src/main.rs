mod matches;

use std::process::exit;
use clap::{Parser, Args, Subcommand};
use anyhow::{Context, Result};
use matches::find_matches;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version)]
#[command(about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// The pattern to look for
    // pattern: String,
    /// The path to the file to read
    // #[arg()]
    // path: std::path::PathBuf,

    #[command(subcommand)]
    // #[clap(required=false)]
    command: Option<Commands>,

    #[arg(short, long)]
    build: bool,

    #[arg(short='x', long)]
    experimental: bool,
}

#[derive(Subcommand)]
enum Commands {
    Search(Search),
}

#[derive(Args)]
struct Search {
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

    if args.build {
       buildInfo();
        exit(0);
    }

    // match on the commands
    match &args.command {
        // args.command is of type Option, check for both some and none
        Some(Search) => {

            // we have something, now match on its type and handle it
            match &Search {
                Commands::Search(Search) => {
                    println!("{}", Search.pattern);
                    println!("{}", Search.path.display());

                    let content = std::fs::read_to_string(&Search.path)
                        .with_context(|| format!("could not read file `{:?}`", Search.path))?;

                    find_matches(&content, &Search.pattern, &mut std::io::stdout());
                }
            }
        }

        // what's best here, just exit?
        None => {
            println!("wtf!")
        }
    }

    Ok(())
}

fn buildInfo() {
    println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
    println!("git semver: {}", env!("VERGEN_GIT_SEMVER"));
    println!("git branch: {}", env!("VERGEN_GIT_BRANCH"));
    println!("git SHA SHORT: {}", env!("VERGEN_GIT_SHA_SHORT"));
}
