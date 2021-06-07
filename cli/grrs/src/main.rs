use structopt::StructOpt;
use std::io::{self, Write};
use anyhow::{Context, Result};
use log::{info, warn};

// Below is documentation <3 rust <3
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    env_logger::init(); // Use environment variable RUST_LOG to control logging level
    info!("starting up");
    warn!("oops, this is a test warning!");

    // Print arguments
    let path_str = args.path.as_path().to_str().unwrap();
    println!("Pattern: {}, Path: {}", args.pattern, path_str);

    // Read file
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path_str))?;

    //println!("file content: {}", content);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer

    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line)?;
        }
    }

    handle.flush()?;

    Ok(())
}