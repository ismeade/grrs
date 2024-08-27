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

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            let result = writeln!(writer, "{}", line);
            if let Err(error) = result {
                panic!("could not write io {}", error)
            }
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

