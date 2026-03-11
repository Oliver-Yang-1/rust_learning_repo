use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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
        self.sender.send(Message::NewJob(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    #[allow(dead_code)]
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => job(),
                    Message::Terminate => break,
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ThreadPool;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, mpsc};
    use std::time::Duration;

    #[test]
    fn drop_waits_for_running_jobs_to_finish() {
        let completed = Arc::new(AtomicUsize::new(0));

        {
            let pool = ThreadPool::new(2);

            for _ in 0..2 {
                let completed = Arc::clone(&completed);
                pool.execute(move || {
                    std::thread::sleep(Duration::from_millis(50));
                    completed.fetch_add(1, Ordering::SeqCst);
                });
            }
        }

        assert_eq!(completed.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn all_jobs_complete_before_shutdown_returns() {
        let (tx, rx) = mpsc::channel();

        {
            let pool = ThreadPool::new(2);
            for id in 0..2 {
                let tx = tx.clone();
                pool.execute(move || {
                    tx.send(id).unwrap();
                });
            }
        }

        let mut results = vec![
            rx.recv_timeout(Duration::from_secs(1)).unwrap(),
            rx.recv_timeout(Duration::from_secs(1)).unwrap(),
        ];
        results.sort();
        assert_eq!(results, vec![0, 1]);
    }
}
