use super::super::models::api_models::WizardMessage;
use crate::models::db_models::Contract;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use log::{error};

// Compilation Request structure
pub struct CompilationRequest {
    pub wizard_message: WizardMessage,
    pub code_id: String,
    pub tx: mpsc::Sender<Result<Contract, String>>,
}

// Compilation Queue is a thread-safe queue that holds CompilationRequests
pub struct CompilationQueue {
    pub queue: Arc<Mutex<Vec<CompilationRequest>>>,
}

// Compilation Queue implementation
impl CompilationQueue {
    // Create a new CompilationQueue
    pub fn new() -> CompilationQueue {
        CompilationQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Add a CompilationRequest to the queue
    pub fn add_request(&self, request: CompilationRequest) {
        let queue_res = self.queue.lock();

        if queue_res.is_err() {
            error!(target: "compiler", "Error locking queue");
            return;
        }

        let mut queue = queue_res.expect("This will never panic because we checked for errors before");

        queue.push(request);
    }

    // Take a CompilationRequest from the queue
    pub fn take_request(&self) -> Option<CompilationRequest> {
        let queue_res = self.queue.lock();

        if queue_res.is_err() {
            error!(target: "compiler", "Error locking queue");
            return None;
        }

        let mut queue = queue_res.expect("This will never panic because we checked for errors before");
        
        if queue.is_empty() {
            None
        } else {
            Some(queue.remove(0))
        }
    }
}
