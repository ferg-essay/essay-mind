/// Thread grouping of tickers
/// 
/// 

use std::{thread, rc::Rc, cell::RefCell, sync::{Arc, Mutex}};

pub struct Task {
    value: i32,
}

impl Task {
    pub fn run(&mut self) {
        self.value += 1;
        println!("hello {}", self.value);
    }
}

pub struct TickerThread {
    tasks: Vec<Task>,
}

impl TickerThread {
    fn push(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn run(&mut self) {
        for _ in 0..10 {
            for task in &mut self.tasks {
                task.run();
            }
        }
    }
}

pub struct ThreadManager {
}

impl ThreadManager {
    pub fn spawn(&self) {
        let mut task = Task { value : 3, };
        let mut manager = TickerThread { tasks: Vec::new() };
        manager.push(task);

        let thread = thread::spawn(move || manager.run());

        thread.join();
    }
}

pub fn test_thread() {
}