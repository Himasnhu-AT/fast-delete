use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    if !path.exists() {
        eprintln!("Directory does not exist: {}", path.display());
        std::process::exit(1);
    }

    println!("Scanning directory...");

    // Count total files first
    let total_files = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count();

    println!("Found {} files to delete", total_files);

    let counter = Arc::new(AtomicUsize::new(0));
    let start_time = Instant::now();

    // Create progress bar
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    // Collect all files first
    let files: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    // Delete files in parallel
    files.par_iter().for_each(|entry| {
        let path = entry.path();
        if let Err(e) = std::fs::remove_file(path) {
            eprintln!("Error deleting {}: {}", path.display(), e);
        } else {
            counter.fetch_add(1, Ordering::Relaxed);
            pb.inc(1);
            pb.set_message(format!("Current: {}", path.display()));
        }
    });

    pb.finish_with_message("File deletion complete");

    // Remove empty directories
    println!("\nRemoving empty directories...");
    let mut dirs_removed = 0;
    for entry in WalkDir::new(path)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            match std::fs::remove_dir(entry.path()) {
                Ok(_) => {
                    println!("Removed directory: {}", entry.path().display());
                    dirs_removed += 1;
                }
                Err(_) => {
                    // Skip directories that can't be removed (probably not empty)
                    continue;
                }
            }
        }
    }

    let duration = start_time.elapsed();
    println!("\nDeletion Summary:");
    println!("Files deleted: {}", counter.load(Ordering::Relaxed));
    println!("Directories removed: {}", dirs_removed);
    println!("Time taken: {:.2?}", duration);
}
