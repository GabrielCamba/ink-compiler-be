use super::super::models::api_models::WizardMessage;
use crate::models::db_models::Contract;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

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
        let mut queue = self.queue.lock().expect("Error locking queue");
        queue.push(request);
    }

    // Take a CompilationRequest from the queue
    pub fn take_request(&self) -> Option<CompilationRequest> {
        let mut queue = self.queue.lock().expect("Error locking queue");
        if queue.is_empty() {
            None
        } else {
            Some(queue.remove(0))
        }
    }
}
