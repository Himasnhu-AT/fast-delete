use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use walkdir::WalkDir;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Message>>,
}

struct Worker {
    _thread: thread::JoinHandle<()>,
}

enum Message {
    File(PathBuf),
    Terminate,
}

impl ThreadPool {
    fn new(size: usize, progress: Arc<Mutex<ProgressBar>>, counter: Arc<Mutex<u64>>) -> ThreadPool {
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            let progress = Arc::clone(&progress);
            let counter = Arc::clone(&counter);

            let thread = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::File(file) => {
                        if let Err(e) = std::fs::remove_file(&file) {
                            eprintln!("Thread {} error deleting {}: {}", id, file.display(), e);
                        } else {
                            {
                                let mut count = counter.lock().unwrap();
                                *count += 1;
                            }
                            progress.lock().unwrap().inc(1);
                            progress
                                .lock()
                                .unwrap()
                                .set_message(format!("Deleting: {}", file.display()));
                        }
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            });

            workers.push(Worker { _thread: thread });
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    fn execute(&self, message: Message) {
        self.sender.as_ref().unwrap().send(message).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender
                .as_ref()
                .unwrap()
                .send(Message::Terminate)
                .unwrap();
        }

        for worker in &mut self.workers {
            let _ = std::mem::replace(&mut worker._thread, thread::spawn(|| {})).join();
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory> [num_threads]", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let num_threads = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| num_cpus::get());

    if !path.exists() {
        eprintln!("Directory does not exist: {}", path.display());
        std::process::exit(1);
    }

    println!("Scanning directory...");
    let start_time = Instant::now();

    // Count files first
    let total_files = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count();

    println!(
        "Found {} files to delete using {} threads",
        total_files, num_threads
    );

    // Setup progress bar
    let progress = ProgressBar::new(total_files as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    let progress = Arc::new(Mutex::new(progress));
    let counter = Arc::new(Mutex::new(0u64));

    // Create thread pool
    let pool = ThreadPool::new(num_threads, Arc::clone(&progress), Arc::clone(&counter));

    // Process files
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        pool.execute(Message::File(entry.path().to_path_buf()));
    }

    // Thread pool will be dropped here, ensuring all files are processed

    progress
        .lock()
        .unwrap()
        .finish_with_message("File deletion complete");

    // Remove empty directories
    println!("\nRemoving empty directories...");
    let mut dirs_removed = 0;
    for entry in WalkDir::new(path)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        if let Ok(()) = std::fs::remove_dir(entry.path()) {
            println!("Removed directory: {}", entry.path().display());
            dirs_removed += 1;
        }
    }

    let duration = start_time.elapsed();
    let total_deleted = *counter.lock().unwrap();

    println!("\nDeletion Summary:");
    println!("Files processed: {}/{}", total_deleted, total_files);
    println!("Directories removed: {}", dirs_removed);
    println!("Time taken: {:.2?}", duration);
    println!(
        "Average speed: {:.2} files/second",
        total_deleted as f64 / duration.as_secs_f64()
    );
}
