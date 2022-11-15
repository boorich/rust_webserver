use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
} // this is a struct that holds a vector of JoinHandle values

type Job = Box<dyn FnOnce() + Send + 'static>; // this is a type alias for a trait object that implements the FnOnce trait

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

        let (sender, receiver) = mpsc::channel(); // create a channel to send messages to the threads

        let receiver = Arc::new(Mutex::new(receiver)); // create a mutex to make sure that only one thread is trying to get a job at a time

        let mut workers = Vec::with_capacity(size); // initialize a vector of workers as a fixed-size array which is more efficient than a vector

        for id in 0..size { // iterate over the size of the thread pool
            workers.push(Worker::new(id, Arc::clone (&receiver))); // push a new worker to the vector
        }

        ThreadPool { workers, sender } // return the ThreadPool struct
    }

    /// Executes the given closure in a thread.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // this trait bound is required for the closure to be able to be executed in a thread
    {
        let job = Box::new(f); // create a boxed closure

        self.sender.send(job).unwrap(); // send the closure to the channel
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>, // join handle is a value that represents a handle to a spawned thread
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker { // instantiate a new worker private function since it's only used in the ThreadPool struct
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap(); // lock the receiver and get the job

            println!("Worker {id} got a job; executing."); // print a message to the console

            job(); // execute the job
        }); // spawn a new thread and move the receiver into the closure to be able to use it

        Worker { id, thread }
    }
}