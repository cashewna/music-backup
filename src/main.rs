use dirs::home_dir;
use std::fs::{self};
use std::time::Instant;
use music_backup::process_dirs;

fn main() {
    let start = Instant::now();

    let music_dir = home_dir().unwrap().join("Downloads");
    let output_dir = home_dir().unwrap().join("music_bak");

    fs::create_dir_all(&output_dir).expect("Failed to create output dir.");

    process_dirs(&music_dir, &output_dir).expect("Failed to process directories.");

    let duration = start.elapsed();
    println!(
        "Took {}.{} seconds",
        duration.as_secs(),
        duration.subsec_nanos()
    );
}

