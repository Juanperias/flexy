use std::sync::{Mutex, OnceLock};

use crate::models::job::Job;

pub static JOBS: OnceLock<Mutex<Vec<Job>>> = OnceLock::new();

pub fn init_globals() {
    JOBS.set(Vec::new().into())
        .expect("Error: Mutex cannot be initialized");
}
