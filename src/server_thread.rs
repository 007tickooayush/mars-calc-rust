use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: tokio::sync::mpsc::Sender<Job>,
}

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    pub fn new() -> ThreadPool {
        let (sender, mut receiver) = tokio::sync::mpsc::channel(1024);
        let receiver = Arc::new(Mutex::new(receiver));

        let total_cpus = match std::thread::available_parallelism() {
            Ok(value) => {
                value.get()
            }
            Err(e) => {
                eprintln!("Failed to get the number of CPUs: {}", e);
                0
            }
        };


        let mut workers = Vec::with_capacity(total_cpus);
        for id in 0..total_cpus {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender,
        }
    }

    pub async fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).await.unwrap();
    }
}

struct Worker {
    id: usize,
    thread: tokio::task::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<Job>>>) -> Worker {
        let thread = tokio::spawn(async move {
            loop {
                let job = receiver.lock().await.recv().await.unwrap();
                println!("\n\nWorker Thread {id} executing request");
                job();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}