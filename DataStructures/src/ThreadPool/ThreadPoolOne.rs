
pub mod thread_pool_one
{
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;

    type Task = Box<dyn FnOnce() + Send + 'static>;

    struct ThreadPool
    {
        workers: Vec<thread::JoinHandle<()>>,
        task_queue: Arc<Mutex<Vec<Task>>>,
    }

    impl ThreadPool
    {
        fn new(size: usize) -> Self
        {
            let task_queue: Arc<Mutex<Vec<Task>>> = Arc::new(Mutex::new(Vec::new()));
            let mut workers: Vec<JoinHandle<()>>  = Vec::with_capacity(size);

            // Create the worker threads
            for id in 0..size {
                let queue: Arc<Mutex<Vec<Task>>> = Arc::clone(&task_queue);
                let handle: JoinHandle<()> = thread::spawn(move || {
                    println!("Worker {} starting", id);
                    loop  {  // Try to get a task from the queue
                        let task: Option<Task> = {
                            let mut queue: MutexGuard<Vec<Task>> = queue.lock().unwrap();
                            queue.pop()
                        };

                        match task {
                            Some(task) => {
                                println!("Worker {} got a task", id);
                                task();
                            }
                            None => {  // No tasks available, sleep a bit
                                thread::sleep(Duration::from_millis(100));
                            }
                        }
                    }
                });
                workers.push(handle);
            }

            ThreadPool { workers, task_queue }
        }

        fn postTask<F>(&self, f: F)
            where F: FnOnce() + Send + 'static,
        {
            let task: Box<F> = Box::new(f);
            self.task_queue.lock().unwrap().push(task);
        }
    }


    pub fn demo()
    {
        let pool: ThreadPool = ThreadPool::new(4);

        // Add tasks to the pool
        for i in 0..10 {
            pool.postTask(move || {
                println!("Executing task {}", i);
                thread::sleep(Duration::from_millis(500));
            });
        }

        // Wait to see the results
        thread::sleep(Duration::from_secs(3));
    }
}