use std::{
    thread::{Thread, self, JoinHandle}, 
    sync::{mpsc::{self, Receiver, Sender}, Arc}, 
    time::Duration
};

use crossbeam_deque::{Worker, Stealer, Injector, Steal};

use super::schedule::SystemId;

pub struct ThreadPoolBuilder {
    n_threads: Option<usize>,
}

type MainClosure = Box<dyn FnOnce(&TaskSender) + Send>;
type TaskClosure = Box<dyn FnOnce() -> SystemId + Send>;

pub struct ThreadPool {
    threads: Vec<Thread>,
    executive: Option<JoinHandle<()>>,

    executive_sender: Sender<MainMessage>,
    executive_reader: Receiver<MainMessage>,
}

enum MainMessage {
    Start(MainClosure),
    Complete,
    Exit,
}

enum TaskMessage {
    Start(TaskClosure),
    Exit,
}

pub struct ExecutiveThread {
    main_reader: Receiver<MainMessage>,
    main_sender: Sender<MainMessage>,

    registry: Arc<Registry>,

    task_receiver: Receiver<SystemId>,
    handles: Vec<JoinHandle<()>>,
}

struct Registry {
    injector: Injector<TaskMessage>,
    tasks: Vec<TaskInfo>,
}

struct TaskInfo {
    stealer: Stealer<TaskMessage>,
    handle: Option<JoinHandle<()>>,
}

impl TaskInfo {
    pub fn new(stealer: Stealer<TaskMessage>) -> Self {
        TaskInfo {
            stealer,
            handle: None,
        }
    }
}

struct TaskThread {
    registry: Arc<Registry>,
    worker: Worker<TaskMessage>,
    sender: Sender<SystemId>,
    index: usize,
}

impl ThreadPoolBuilder {
    pub fn new() -> Self {
        Self {
            n_threads: None,
        }
    }

    pub fn n_threads(&mut self, n_threads: usize) -> &mut Self {
        assert!(n_threads > 0);

        self.n_threads = Some(n_threads);

        self
    }

    pub fn build(self) -> ThreadPool {
        let parallelism = thread::available_parallelism().unwrap();
        println!("parallel {:?}", parallelism);

        let (executive_sender, main_reader) = mpsc::channel();
        let (main_sender, executive_reader) = mpsc::channel();

        let (task_sender, task_reader) = mpsc::channel();

        let n_threads = match self.n_threads {
            Some(n_threads) => n_threads,
            None => 2,
        };

        let mut registry = Registry {
            injector: Injector::new(),
            tasks: Vec::new(),
        };

        let mut workers = Vec::new();
        for _ in 0..n_threads {
            let worker = Worker::new_fifo();

            registry.tasks.push(TaskInfo::new(worker.stealer()));

            workers.push(worker);
        }

        let registry = Arc::new(registry);
        let mut handles = Vec::<JoinHandle<()>>::new();

        for i in 0..n_threads {
            let mut task_thread = TaskThread::new(
                Arc::clone(&registry), 
                workers.remove(0), 
                task_sender.clone(),
                i
            );

            let handle = thread::spawn(move || {
                task_thread.run();
            });

            handles.push(handle);
        }

        let mut executive = ExecutiveThread {
            main_reader,
            main_sender,

            registry,

            task_receiver: task_reader,
            handles,
        };

        let handle = thread::spawn(move || {
            executive.run();
        });

        ThreadPool {
            threads: Vec::new(),

            executive: Some(handle),

            executive_sender,
            executive_reader,
        }
    }
}

impl ThreadPool {
    pub fn start(&mut self, fun: impl FnOnce(&TaskSender) + Send + 'static) {
        self.executive_sender.send(MainMessage::Start(Box::new(fun))).unwrap();
        
        loop {
            match self.executive_reader.recv() {
                Ok(MainMessage::Exit) => {
                    self.close();
                    panic!("unexpected exit");
                }
                Ok(MainMessage::Complete) => {
                    return;
                }
                Ok(_) => {
                    panic!("invalid executive message");
                }
                Err(err) => {
                    panic!("executor receive error {:?}", err);
                }
            }
        }
    }

    pub fn close(&mut self) {
        match self.executive.take() {
            Some(handle) => {
                self.executive_sender.send(MainMessage::Exit).unwrap();
                // TODO: timed?
                handle.join().unwrap();
            },
            None => {},
        }
    }
}

pub struct TaskSender<'a> {
    thread: &'a ExecutiveThread,
}

impl<'a> TaskSender<'a> {
    pub fn send(&self, fun: impl FnOnce() -> SystemId) {

    }
}


impl ExecutiveThread {
    pub fn run(&mut self) {
        let sender = TaskSender { thread: &self };
        loop {
            match self.main_reader.recv() {
                Ok(MainMessage::Start(task)) => {
                    task(&sender);

                    self.main_sender.send(MainMessage::Complete).unwrap();
                }
                Ok(MainMessage::Exit) => {
                    self.main_sender.send(MainMessage::Exit).unwrap();
                    return;
                }
                Ok(_) => {
                    panic!("invalid executor message");
                }
                Err(err) => {
                    panic!("executor receive error {:?}", err);
                }
            }
        }
    }
}

impl TaskThread {
    pub fn new(
        registry: Arc<Registry>, 
        worker: Worker<TaskMessage>,
        sender: Sender<SystemId>,
        index: usize
    ) -> Self {
        Self {
            registry,
            worker,
            sender,
            index,
        }
    }

    pub fn run(&mut self) {
        let worker = &self.worker;
        let injector = &self.registry.injector;

        loop {
            let msg = match worker.pop() {
                Some(msg) => msg,
                None => {
                    match injector.steal() {
                        Steal::Success(msg) => msg,
                        crossbeam_deque::Steal::Retry => {
                            continue;
                        },
                        crossbeam_deque::Steal::Empty => {
                            thread::park();
                            continue;
                        }
                    }
                }
            };

            match msg {
                TaskMessage::Start(fun) => {
                    let id = fun();
                    self.sender.send(id).unwrap();
                },
                TaskMessage::Exit => {
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::ThreadPoolBuilder;

    #[test]
    fn test() {
        let mut pool = ThreadPoolBuilder::new().build();

        println!("pre sleep");
        //thread::sleep(Duration::from_millis(1000));
        println!("post sleep");
        pool.start(|r| println!("hello"));
        println!("pre-close");
        pool.close();
    }
}