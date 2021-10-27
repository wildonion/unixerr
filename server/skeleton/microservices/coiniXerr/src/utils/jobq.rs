





//-- synchronous task scheduler using multiple threads or workers communication based on mpsc job queue channel protocol







use std::thread;
use std::sync::mpsc; //-- communication between threads is done using mpsc channel and end of the channel can only be owned by one thread at the time to avoid dead lock, however the sender half can be cloned and through such cloning the conceptual sender part of a channel can be shared among threads which is how you do the multi-producer, single-consumer part
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;






type Job = Box<dyn FnOnce() + Send + 'static>; //-- a job is of type closure which must be Send and static across all threads inside a Box on the heap

struct Worker{
    id: Uuid,
    thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool{
    
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size); //-- capacity is not always equals to the length and the capacity of this vector is same as the maximum size based on the system arch, on 32 bits arch usize is 4 bytes and on 64 bits arch usize is 8 bytes
        for _ in 0..size { //-- since the receiver is not bounded to trait Clone we must clone it using Arc in each iteration cause we want to share it between multiple threads to get what the sender has sent 
            workers.push(Worker::new(Uuid::new_v4(), Arc::clone(&receiver)));
        }
        ThreadPool{workers, sender}
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap(); //-- by executing the task handler sender will send a job and only one receiver at a time can get that job and solve it by locking on that job inside the choosen thread since thread safe programming is all about this pattern!
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) { //-- destructor for ThreadPool struct 
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take(){ //-- take() takes the value out of the option, leaving a None in its place
                thread.join().unwrap();
            }
        }
    }
}

impl Worker{
    fn new(id: Uuid, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            if let Ok(message) = receiver.lock().unwrap().recv(){ //-- since other thread shouldn't mutate this message while thsi thread is mutating we must do a locking on the message received from the sender to acquire the mutex by blocking the current thread to avoid being in dead lock situation
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job(); //-- this might be a task or job of calling a heavy computational function of a process
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }  
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
