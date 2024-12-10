use indicatif::ProgressBar;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

/// A thread pool for executing tasks concurrently.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Message>>,
}

/// A worker that executes tasks in a separate thread.
struct Worker {
    _thread: thread::JoinHandle<()>,
}

/// Messages that can be sent to the workers.
pub enum Message {
    File(PathBuf),
    Terminate,
}

impl ThreadPool {
    /// Creates a new thread pool with the specified number of threads.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of threads in the pool.
    /// * `progress` - A shared progress bar for tracking progress.
    /// * `counter` - A shared counter for tracking the number of processed files.
    ///
    /// # Returns
    ///
    /// A new `ThreadPool` instance.
    pub fn new(
        size: usize,
        progress: Arc<Mutex<ProgressBar>>,
        counter: Arc<Mutex<u64>>,
    ) -> ThreadPool {
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

    /// Executes a task by sending a message to the thread pool.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send to the thread pool.
    pub fn execute(&self, message: Message) {
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
