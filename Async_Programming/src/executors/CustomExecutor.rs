

mod custom_executor
{
    use std::{
        future::Future, pin::Pin,
        sync::{ mpsc::{sync_channel, Receiver, SyncSender},  Arc, Mutex },
        task::{ Context, Poll, RawWaker, RawWakerVTable, Waker }
    };

    struct Task {
        future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
        sender: SyncSender<Arc<Task>>,
    }

    struct Executor {
        receiver: Receiver<Arc<Task>>,
    }

    struct Spawner {
        sender: SyncSender<Arc<Task>>,
    }

    impl Executor
    {
        fn run(&self)
        {
            while let Ok(task) = self.receiver.recv()
            {
                let mut future_slot = task.future.lock().unwrap();
                if let Some(mut future) = future_slot.take() {
                    let waker: Waker = task.clone().into_waker();
                    let mut cx: Context = Context::from_waker(&waker);

                    match future.as_mut().poll(&mut cx) {
                        Poll::Ready(()) => {}
                        Poll::Pending => {
                            *future_slot = Some(future);
                        }
                    }
                }
            }
        }
    }

    impl Spawner
    {
        fn spawn(&self, future: impl Future<Output = ()> + Send + 'static)
        {
            let future = Box::pin(future);
            let task: Arc<Task> = Arc::new(Task {
                future: Mutex::new(Some(future)),
                sender: self.sender.clone(),
            });
            self.sender.send(task).unwrap();
        }
    }

    impl Task
    {
        fn into_waker(self: Arc<Self>) -> Waker {
            unsafe { Waker::from_raw(Self::raw_waker(self)) }
        }

        fn raw_waker(task: Arc<Self>) -> RawWaker {
            RawWaker::new(
                Arc::into_raw(task) as *const (),
                &RawWakerVTable::new(
                    Self::clone,
                    Self::wake,
                    Self::wake_by_ref,
                    Self::drop,
                ),
            )
        }

        unsafe fn clone(ptr: *const ()) -> RawWaker
        {
            let arc: Arc<Task> = Arc::from_raw(ptr as *const Task);
            let cloned: Arc<Task> = arc.clone();
            std::mem::forget(arc);
            Self::raw_waker(cloned)
        }

        unsafe fn wake(ptr: *const ())
        {
            let arc: Arc<Task> = Arc::from_raw(ptr as *const Task);
            arc.sender.send(arc.clone()).unwrap();
        }

        unsafe fn wake_by_ref(ptr: *const ())
        {
            let arc: Arc<Task> = Arc::from_raw(ptr as *const Task);
            arc.sender.send(arc.clone()).unwrap();
            std::mem::forget(arc);
        }

        unsafe fn drop(ptr: *const ()) {
            drop(Arc::from_raw(ptr as *const Task));
        }
    }

    fn new_executor() -> (Spawner, Executor)
    {
        let (sender, receiver) = sync_channel(1024);
        return (
            Spawner { sender: sender.clone() },
            Executor { receiver }
        )
    }

    async fn hello() {
        println!("Hello");
    }

    pub fn demo()
    {
        let (spawner, executor) = new_executor();
        spawner.spawn(hello());
        drop(spawner);
        executor.run();
    }
}


pub fn testAll()
{
    custom_executor::demo();
}