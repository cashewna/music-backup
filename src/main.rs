use dirs::home_dir;
use music_backup::process_dirs;
use std::fs::{self};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let music_dir = home_dir().unwrap().join("Downloads");
    let output_dir = home_dir().unwrap().join("music_bak");

    fs::create_dir_all(&output_dir).expect("Failed to create output dir.");

    let (num_files, music_files) =
        process_dirs(&music_dir, &output_dir).expect("Failed to process directories.");

    let duration = start.elapsed();
    println!(
        "Proccessed {num_files} file(s) in {}.{} seconds. Copied {music_files} song(s).",
        duration.as_secs(),
        duration.subsec_nanos()
    );
}
