use std::thread;
use std::thread::Thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);
        
        for _ in 0..size {
            // TODO スレッドを生成してベクタを格納する
        }
        
        ThreadPool {
            threads
        }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {

    }
}