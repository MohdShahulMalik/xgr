* Indicatif Crate

The indicatif crate is a popular and powerful Rust library for creating highly customizable progress bars, spinners, and other progress indicators in command-line applications.

Basic Usage Examples

First, add indicatif to your Cargo.toml:

```
[dependencies]
indicatif = "0.17"
```

1. Simple Progress Bar

This is the most common use case for a task with a known number of steps.

rust
use indicatif::ProgressBar;
use std::thread;
use std::time::Duration;

let bar = ProgressBar::new(1000);
for _ in 0..1000 {
    bar.inc(1);
    thread::sleep(Duration::from_millis(1));
}
bar.finish();

    ProgressBar::new(1000): Creates a bar with a total length of 1000.

    bar.inc(1): Increments the bar's position by 1.

    bar.finish(): Marks the bar as complete and stops redrawing.

2. Spinner for Unknown Durations

When you don't know how long a task will take, a spinner is a great choice.

rust
use indicatif::ProgressBar;
use std::thread;
use std::time::Duration;

let spinner = ProgressBar::new_spinner();
spinner.set_message("Processing data...");
thread::sleep(Duration::from_secs(3)); // Simulate work
spinner.finish_with_message("Done!");

    ProgressBar::new_spinner(): Creates a progress indicator that cycles through a set of characters.

    set_message(...): Displays a message next to the spinner.

    finish_with_message(...): Replaces the spinner with a final message.

3. Styling and Templating

indicatif's real power comes from its templating engine. You can control every aspect of the bar's appearance.

rust
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

let bar = ProgressBar::new(100);
bar.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .expect("Failed to parse template")
    .progress_chars("##-"));

for i in 0..100 {
    bar.set_message(format!("item #{}", i));
    bar.inc(1);
    thread::sleep(Duration::from_millis(50));
}
bar.finish();

Let's break down the template [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}:

    {elapsed_precise}: Shows the elapsed time (e.g., 00:00:05.123).

    {bar:40.cyan/blue}: A 40-character wide bar. The completed portion is cyan, and the remaining portion is blue.

    {pos:>7}: The current position, right-aligned to a width of 7 characters.

    {len:7}: The total length, aligned to a width of 7 characters.

    {msg}: A custom message set via bar.set_message().

    .progress_chars("##-"): Defines the characters used for the filled, current, and remaining parts of the bar.

Advanced Features
Multi-Progress Bars

For managing progress of multiple concurrent tasks, MultiProgress is invaluable. It ensures that multiple bars can be drawn without interfering with each other.

rust
use indicatif::{MultiProgress, ProgressBar};
use std::thread;
use std::time::Duration;

let m = MultiProgress::new();
let bar1 = m.add(ProgressBar::new(100));
let bar2 = m.add(ProgressBar::new(200));

let h1 = thread::spawn(move || {
    for _ in 0..100 {
        bar1.inc(1);
        thread::sleep(Duration::from_millis(20));
    }
    bar1.finish();
});

let h2 = thread::spawn(move || {
    for _ in 0..200 {
        bar2.inc(1);
        thread::sleep(Duration::from_millis(15));
    }
    bar2.finish();
});

m.join_and_clear().unwrap();

Wrapping Iterators

You can easily add a progress bar to any iterator using the ProgressIterator trait.

rust
use indicatif::ProgressIterator;

let sum: u64 = (0..1000)
    .progress_count(1000)
    .map(|x| x as u64)
    .sum();

This will automatically create a progress bar that advances as the iterator is consumed.
