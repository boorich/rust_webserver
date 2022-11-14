pub struct ThreadPool; // this is a placeholder for the actual struct

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // this is shorter than if size == 0 { panic!() }

        ThreadPool
    }

    /// Executes the given closure in a thread.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // this trait bound is required for the closure to be able to be executed in a thread
    {
    }
}