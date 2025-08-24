use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    query: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file {}", args.path.display()))?;
    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("Can't read line {}", line_num))?;
        if line.contains(&args.query) {
            println!("{}: {}", line_num + 1, line);
        }
    }

    Ok(())
}
