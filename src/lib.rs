use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
} // this is a struct that holds a vector of JoinHandle values

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // just to make sure that the size is greater than 0 or else panic

        let mut workers = Vec::with_capacity(size); // initialize a vector of workers as a fixed-size array which is more efficient than a vector

        for id in 0..size { // iterate over the size of the thread pool
            workers.push(Worker::new(id)); // push a new worker to the vector
        }

        ThreadPool { workers } // return the ThreadPool struct
    }

    /// Executes the given closure in a thread.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // this trait bound is required for the closure to be able to be executed in a thread
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>, // join handle is a value that represents a handle to a spawned thread
}

impl Worker {
    fn new(id: usize) -> Worker { // instantiate a new worker private function since it's only used in the ThreadPool struct
        let thread = thread::spawn(|| {}); // spawm an empty thread to be used by the worker to execute a closure

        Worker { id, thread }
    }
}