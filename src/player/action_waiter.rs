use std::collections::HashSet;
use std::sync::Arc;

use tokio::sync::{Mutex, Notify};

#[derive(Clone)]
pub struct ActionPrefixWaiter {
    values: Arc<Mutex<WaiterInner>>,
    notify: Arc<Notify>,
}


pub struct WaiterInner {
    // expect start from 0
    finish_prefix: u64,
    set : HashSet<u64>,
}


impl ActionPrefixWaiter {
    pub fn new() -> Self {
        Self {
            values: Arc::new(Mutex::new(WaiterInner::new())),
            notify: Arc::new(Notify::new()),
        }
    }

    // index expect start from 0
    pub async fn wait_finish_prefix(&self, index:u64) {
        let mut guard = self.values.lock().await;
        loop {
            if guard.finish_prefix(index) {
                break;
            } else {
                drop(guard);
                self.notify.notified().await;
                guard = self.values.lock().await;
            }
        }
    }

    pub async fn finish_one(&self, index:u64) {
        let mut guard = self.values.lock().await;
        guard.finish_one(index);
        self.notify.notify_waiters();
    }
}

impl WaiterInner {
    fn new() -> Self {
        Self {
            finish_prefix:0,
            set : HashSet::new()
        }
    }

    fn finish_prefix(&self, index:u64) -> bool {
       self.finish_prefix >= index
    }

    fn finish_one(&mut self, index:u64) {
        if self.finish_prefix == index {
            self.finish_prefix += 1;
            while self.continue_next_one() {};
        } else if self.finish_prefix < index {
            let _ = self.set.insert(index);
        }
    }

    fn continue_next_one(&mut self) -> bool {
        if self.set.contains(&self.finish_prefix) {
            self.set.remove(&self.finish_prefix);
            self.finish_prefix += 1;
            true
        } else {
            false
        }
    }
}