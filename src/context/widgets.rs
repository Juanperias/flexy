use crate::{
    context::globals::JOBS,
    models::job::{Job, Kind},
};

pub fn text(value: String) {
    let mut jobs = JOBS.get().expect("Error: cannot open jobs").lock().unwrap();

    jobs.push(Job {
        kind: Kind::Text,
        value,
    });
}
