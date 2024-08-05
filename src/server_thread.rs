
struct ThreadPool {
    workers: Vec<Worker>
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {

    }
}

struct Worker {
    id: usize,
    thread: std::thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize) -> Worker {
        unimplemented!()
    }
}