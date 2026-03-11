use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "thread pool size must be greater than zero");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }

    pub fn size(&self) -> usize {
        self.workers.len()
    }
}

struct Worker {
    #[allow(dead_code)]
    id: usize,
    #[allow(dead_code)]
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                job();
            }
        });

        Self { id, thread }
    }
}

#[cfg(test)]
mod tests {
    use super::ThreadPool;
    use std::sync::{Arc, Mutex, mpsc};
    use std::time::Duration;

    #[test]
    #[should_panic(expected = "thread pool size must be greater than zero")]
    fn new_panics_when_size_is_zero() {
        let _ = ThreadPool::new(0);
    }

    #[test]
    fn execute_runs_multiple_jobs() {
        let pool = ThreadPool::new(2);
        let results = Arc::new(Mutex::new(Vec::new()));
        let (done_tx, done_rx) = mpsc::channel();

        for label in ["job-a", "job-b", "job-c"] {
            let results = Arc::clone(&results);
            let done_tx = done_tx.clone();
            pool.execute(move || {
                results.lock().unwrap().push(label.to_string());
                done_tx.send(()).unwrap();
            });
        }

        for _ in 0..3 {
            done_rx.recv_timeout(Duration::from_secs(1)).unwrap();
        }

        let mut values = results.lock().unwrap().clone();
        values.sort();
        assert_eq!(values, vec!["job-a", "job-b", "job-c"]);
    }

    #[test]
    fn pool_reports_worker_count() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.size(), 4);
    }
}
