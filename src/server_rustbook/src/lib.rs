use std::usize;

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(usize: usize) -> ThreadPool {
        return ThreadPool;
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
