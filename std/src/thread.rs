use std::thread::{self, JoinHandle};

pub struct Thread {
    handle: Option<JoinHandle<()>>,
}

impl Thread {
    pub fn spawn<F>(f: F) -> Self 
    where
        F: FnOnce() + Send + 'static
    {
        Thread {
            handle: Some(thread::spawn(f))
        }
    }

    pub fn join(&mut self) -> thread::Result<()> {
        self.handle.take().unwrap().join()
    }
}