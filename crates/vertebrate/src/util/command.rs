use std::{collections::VecDeque, sync::Mutex};

pub struct Command<T> {
    queue: Mutex<VecDeque<T>>,
}

impl<T> Command<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new())
        }
    }

    pub fn send(&self, msg: T) {
        self.queue.lock().unwrap().push_back(msg);
    }

    pub fn drain(&mut self) -> Vec<T> {
        self.queue.lock().unwrap().drain(..).collect::<Vec<T>>()
    }
}