use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::Thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>
}

struct Job {}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        
        // 複数のスレッドで共有しつつ、可変化させるためにArcを用いる
        // 競合状態を回避するためにMutexを用いる
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {

    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        
        Worker {
            id,
            thread,
        }
    }
}
