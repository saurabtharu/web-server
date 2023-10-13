pub struct ThreadPool;

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new function panics if the size is zero`
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        Self
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        /* We still use the () after FnOnce because this FnOnce represents a closure that takes no
         * parameters and returns the unit type () */
    {
    }
}
