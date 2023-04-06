use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub struct CompileRequest {
    pub code: String,
    pub id: String,
    pub tx: mpsc::Sender<String>,
}

pub struct CompilationQueue {
    pub queue: Arc<Mutex<Vec<CompileRequest>>>,
}

impl CompilationQueue {
    pub fn new() -> CompilationQueue {
        CompilationQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_request(&self, request: CompileRequest) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(request);
        drop(queue);
    }

    pub fn take_request(&self) -> Option<CompileRequest> {
        let mut queue = self.queue.lock().unwrap();
        if queue.is_empty() {
            None
        } else {
            Some(queue.remove(0))
        }
    }

    pub fn start(&self) {}
}
