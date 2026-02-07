
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Value {
    pub id: u16,
    pub timestamp:u64,
    pub value: String,
}

pub struct Queue {
    inner: Arc<Mutex<VecDeque<Value>>>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            inner: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn enqueue(&self, value: Value) {
        let mut queue = self.inner.lock().unwrap();
        queue.push_back(value);
    }

    pub fn dequeue(&self) -> Option<Value> {
        let mut queue = self.inner.lock().unwrap();
        queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        let queue = self.inner.lock().unwrap();
        queue.is_empty()
    }
}
