use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// `ThreadPool` data structure to hold a vector of `Worker` instances
///
/// field: workers -> Vec<Worker>
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new function panics if the size is zero`
    ///
    /// use the `for` loop counter to generate an `id`, create a new `Worker` with that `id`, and store the worker in the vector
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        /* We still use the () after FnOnce because this FnOnce represents a closure that takes no
         * parameters and returns the unit type () */
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

/// `Worker` data structure holds an `id` and a `JoinHandle<()>`
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

/// function that takes an id number and returns a Worker instance that holds the id and a thread spawned with an empty closure
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });
        Worker { id, thread }
    }
}
