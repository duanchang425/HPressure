use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct Stats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl Stats {
    pub fn add_request(&mut self, success: bool, sent: u64, received: u64) {
        self.total_requests += 1;
        self.bytes_sent += sent;
        self.bytes_received += received;
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }
    }
}

pub type StatsArc = Arc<Mutex<Stats>>; 