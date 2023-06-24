use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// struct Job;

impl ThreadPool {
    /// Creates a new thread pool
    ///
    /// the size is the number of threads in the pool
    ///
    /// #Panics
    ///
    /// The `new` function will panic if the pool size is zero
    ///
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        // 共享receiver Mutex
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size as usize);

        // 只能由一个receiver,线程需要共享
        for id in 0..size {
            println!("{}", id);
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
        // match self.sender.send(job) {
        //     Ok(ok) => println!("ok {:?}", ok),
        //     Err(e) => println!("{}", e),
        // }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

// type Job = Box<dyn FnOnce() + Send + 'static>;
type Job = Box<dyn FnBox + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {:?}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // while let Ok(job) = receiver.lock().unwrap().recv() {
            //     println!("Worker: {} got a job; executing.", id);
            //     // 解包后不知道大小 所以用Box来包裹
            //     // (*job)();
            //     job.call_box();
            // }
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing", id)
                }
                Message::Terminate => {
                    println!("Worker {} was told to Terminate; executing", id);
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
