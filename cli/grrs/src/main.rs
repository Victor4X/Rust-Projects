use structopt::StructOpt;

// Below is documentation <3 rust <3
/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let path_str: String = args.path.into_os_string().into_string().unwrap();
    println!("Pattern: {}, Path: {}", args.pattern, path_str);
}