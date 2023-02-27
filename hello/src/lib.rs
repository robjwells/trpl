use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;
type Receiver = Arc<Mutex<mpsc::Receiver<Job>>>;

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

pub struct PoolCreationError;

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

        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();
        ThreadPool { workers, sender: Some(sender) }
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

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.iter_mut() {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Receiver) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                },
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
