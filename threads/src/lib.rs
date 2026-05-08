use std:: {
    thread,
    sync::{mpsc, Arc, Mutex}
};

struct Worker {
    id: usize,
    thread:thread::JoinHandle<()>,
}
impl Worker {
    fn new(id:usize, receiver:Arc<Mutex<mpsc::Receiver<Jop>>>) ->Self{
        let thread = thread::spawn(move|| {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(jop) => jop(),
                    Err(_) => break,
                }
            }
        });
        Self {
            id,
            thread,
        }
    }
}

type Jop = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers:Vec<Worker>,
    sender: mpsc::Sender<Jop>,
}

impl ThreadPool {
    pub fn new(num_threads:usize)->Self {
        assert!(num_threads > 0);
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(num_threads);
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..num_threads {
            let worker = Worker::new(id, receiver.clone());
            workers.push(worker);
        }
        Self {
            workers,
            sender,
        }
    }
}

