use std::thread;
use std::sync::{ mpsc, Arc, Mutex };

pub struct ThreadPool {
    threads : Vec<Worker>,
    sender : Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size : usize ) -> ThreadPool {
        let (sender , recv) = mpsc::channel();
        let recv = Arc::new(Mutex::new(recv));
        let mut threads = Vec::with_capacity(size);
        for id in 0..size {
            //Create a thread and store them.
            threads.push(Worker::new(id , Arc::clone(&recv)));
        }
        ThreadPool {threads , sender : Some(sender)} 
    }
    pub fn execute<F>(&self , f : F) 
    where F : FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.threads {
            println!("Shutting Down the Worker {}" , worker.id);
            if let Some(thread_to_join) = worker.thread.take() {
                thread_to_join.join().unwrap();
            }
        }
    }
}


type Job = Box<dyn FnOnce() + 'static + Send>;

struct Worker {
    id : usize,
    thread : Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id : usize , recv : Arc<Mutex<mpsc::Receiver<Job>>> )-> Worker {
        let spawned_thread = thread::spawn(move || 
            loop{
                let message = recv.lock().unwrap().recv();
                match  message {
                   Ok(job) => { 
                       println!("Worker {} got a job" , id);
                       job();
                   }
                   Err(_) => {
                       println!("Shutting Down this thread {id}");
                       break;
                   }
                }
            }
        );
        Worker {
            id, 
            thread : Some(spawned_thread),
        }
    }
}
