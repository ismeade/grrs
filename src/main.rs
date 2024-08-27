use anyhow::{Context, Ok, Result};

use clap::Parser;


#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {

    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    //
    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    let args = Cli::parse();

    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path)

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    // let result = std::fs::read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => { content },
    //     Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    // };
    //
    // println!("file content: {}", content)

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

