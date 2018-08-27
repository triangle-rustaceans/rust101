use std::sync::{Arc, Mutex, mpsc};
use std::thread;


fn main() {
    let x = Arc::new(Mutex::new(0));
    let x1 = x.clone();
    let x2 = x.clone();
    let thread1 = thread::spawn(move || {
        let mut data = x1.lock().unwrap();
        *data += 219;
    });
    let thread2 = thread::spawn(move || {
        let mut data = x2.lock().unwrap();
        *data += 37;
    });
    thread1.join().unwrap();       
    thread2.join().unwrap();       
    println!("{}", x.lock().unwrap());
}

struct Worker<T> {
    name: String,
    completed: usize,
    inchan: mpsc::Receiver<Msg<T>>,
    outchan: mpsc::Sender<T>,
}

enum Msg<F, T> 
where 
    T: Send + 'static,
    F: FnOnce() -> T,
    F: Send + 'static,
{
    Kill,
    Task(Option<F>), 
}

type Msg<T> = Msg<F: FnOnce() -> T, T>
where F: FnOnce() -> T;

impl Worker {
    pub fn new(name: String, rx: ) {
        Worker { 
            name: String,
            completed: 0,
        }
    }

    pub fn run(&mut self) -> usize {
        loop {
            match self.inchan.recv() {
                Msg::Kill => break self.completed,
                Msg::Task(task) => self.recv(task),
            }
        }
    }

    pub fn recv<F, T>(task: F) -> T 
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        let result = task();
        completed += 1;
        result
    }
}


struct ThreadPool {
    workers: Vec<JoinHandle<usize>>,
}

impl ThreadPool {
    pub fn new(threads: usize) -> ThreadPool {
        let (task_sender, task_receiver) = mpsc::channel();
        ThreadPool {
            workers: (0..threads).map(|n| { 
                let worker = Worker::new(format!("Worker-{}", n));
                move || worker.run()
            })
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn use_pool() {
        let data = Arc::new(Mutex(0));
        let pool = ThreadPool::new(4);
        for i in 0..10000 {
            copy = data.clone();
            pool.spawn(move || {
                let data = copy.lock().unwrap();
                *data += 1
            });
        }
        assert_eq!(data.lock().unwrap(), &10000);
    }
}
