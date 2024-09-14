use crate::{
    context::globals::JOBS,
    models::job::{Job, JobKind},
};

pub fn text(value: String) {
    let mut jobs = JOBS.get().expect("Error: cannot open jobs").lock().unwrap();

    jobs.push(Job {
        kind: JobKind::Text,
        value,
    });
}
