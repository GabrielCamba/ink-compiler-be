use super::super::models::api_models::WizardMessage;
use crate::models::db_models::Contract;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub struct CompilationRequest {
    pub wizard_message: WizardMessage,
    pub code_id: String,
    pub tx: mpsc::Sender<Result<Contract, String>>,
}

pub struct CompilationQueue {
    pub queue: Arc<Mutex<Vec<CompilationRequest>>>,
}

impl CompilationQueue {
    pub fn new() -> CompilationQueue {
        CompilationQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_request(&self, request: CompilationRequest) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(request);
        drop(queue);
    }

    pub fn take_request(&self) -> Option<CompilationRequest> {
        let mut queue = self.queue.lock().unwrap();
        if queue.is_empty() {
            None
        } else {
            Some(queue.remove(0))
        }
    }

    pub fn start(&self) {}
}
