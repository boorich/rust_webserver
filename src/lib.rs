use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
} // this is a struct that holds a vector of workers and a sender. I workes like a router, and the sender is the channel that the workers use to communicate with each other

// the Job type is an abstraction over the closure that we want to execute
type Job = Box<dyn FnOnce() + Send + 'static>; // this is a type alias for a trait object that implements the FnOnce and the Send traits, while also being 'static

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

        for id in 0..size {
            // iterate over the size of the thread pool
            workers.push(Worker::new(id, Arc::clone(&receiver))); // push a new worker to the vector
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        } // return the ThreadPool struct
    }

    /// Executes the given closure in a thread.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // this trait bound is required for the closure to be able to be executed in a thread
    {
        let job = Box::new(f); // create a boxed closure of the given closure to be able to send it to the channel

        self.sender.
        as_ref(). // get a reference to the sender
        unwrap(). // unwrap the sender
        send(job). // send the job to the channel
        unwrap(); // unwrap the result of the send operation
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // drop the sender

        for worker in &mut self.workers { // iterate over the workers
            println!("Shutting down worker {}", worker.id); // print a message

            if let Some(thread) = worker.thread.take() { // optionally take the thread
                thread.join().unwrap(); // join the thread
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>, // optional JoinHandle value that will be None if the thread panics
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
