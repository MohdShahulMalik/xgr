use indicatif::{ProgressBar, ProgressStyle};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::{Context, Result};
use std::thread;
use std::time::Duration;

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
    let bar = ProgressBar::new_spinner();
    bar.set_message("Searching...");

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("Can't read line {}", line_num))?;
        if line.contains(&args.query) {
            println!("{}: {}", line_num + 1, line);
        }

        if line_num != 0 && line_num % 1000 == 0{
            bar.set_message(format!("Searched {} lines", line_num));
        }
    }


//Example 1
    let bar = ProgressBar::new(1000);
    for _ in 0..1000 {
        bar.inc(1);
        thread::sleep(Duration::from_millis(1));
    }
    bar.finish();

//Example 2
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Processing data...");
    thread::sleep(Duration::from_secs(3)); // Simulate work
    spinner.finish_with_message("Done!");

//Example 3
    let bar = ProgressBar::new(100);
    bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:100.cyan/blue}] {pos:>7}/{len:7} {msg}") // Something wrapped in "[]" means that thing would be shown in "[]", in this case, elapsed_precise and the progress bar will be shown in "[]" in the terminal
        .expect("Failed to parse template")
        .progress_chars("##-"));

    for i in 0..100 {
        bar.set_message(format!("item #{}", i));
        bar.inc(1);
        thread::sleep(Duration::from_millis(50));
    }
    bar.finish();

    Ok(())
}
