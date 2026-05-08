use std::thread;
/*
std::thread::functions
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
 */
pub struct ThreadPool
{
    threads: Vec<thread::JoinHandle<()>>,
}


impl ThreadPool {
    pub fn new (num_threads: usize)->Self {
        assert!(num_threads > 0);
        let mut threads = Vec::with_capacity(num_threads);
        for _ in 0..num_threads {
            threads.push(thread::spawn(move || {}));
        }
        Self {
            threads
        }
    }

    pub fn execute<F>(&self, f:F)
    where F:FnOnce() + Send +'static 
    {

    }
}