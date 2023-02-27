use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct PoolCreationError;

type Receiver = Arc<Mutex<mpsc::Receiver<Job>>>;

impl ThreadPool {
    /// Creates a new [`ThreadPool`].
    ///
    /// The `size` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size).map(|id| {
            Worker::new(id, Arc::clone(&receiver))
        }).collect();
        ThreadPool { workers, sender }
    }

    /// Try to create a new [`ThreadPool`].
    ///
    /// The `size` is the number of threads in the pool, as given to [`ThreadPool::new`].
    ///
    /// # Errors
    ///
    /// This function will return an error if size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            Ok(ThreadPool::new(size))
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Receiver) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| {
                receiver;
            }),
        }
    }
}

struct Job;
