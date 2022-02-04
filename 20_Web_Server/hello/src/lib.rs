use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{JoinHandle, spawn};
use crate::Message::NewJob;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message{
    NewJob(Job),
    Terminal
}


pub struct PoolCreationError {
    pub info: String,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。

    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {} else {
            return Err(PoolCreationError {
                info: format!("The number of threads is {}\n create failed", size)
            });
        }

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            //     循环生成id的Worker实例
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }

        Ok(ThreadPool {
            workers,
            sender,
        })
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(NewJob(job)).unwrap_or_else(|error| {
            eprintln!("{}", error)
        })
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        for worker in &mut self.workers{
            println!("Worker {} terminating...", worker.id);
            self.sender.send(Message::Terminal).unwrap();
        }

        println!("All workers had been told terminated");

        for worker in &mut self.workers{
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();



                if let NewJob(job) = message{
                    println!("Worker {} got a job; executing.", id);
                    job();
                }else{
                    println!("Worker {} was told to terminate", id);
                    break;
                }
            }
        });


        Worker {
            id,
            thread:Some(thread),
        }
    }
}


