use crate::models::styles::Style;
use crate::{
    context::globals::JOBS,
    models::job::{Job, Kind},
};
use mlua::Table;

pub fn clear() {
    let mut jobs = JOBS.get().expect("Error: cannot open jobs").lock().unwrap();
    *jobs = Vec::new();
}

pub fn text(value: String, table: Option<Table>) {
    let mut jobs = JOBS.get().expect("Error: cannot open jobs").lock().unwrap();
    if let Some(safe_table) = table {
        let style = Style::from_table(safe_table).expect("Error: cannot convert the style");

        jobs.push(Job {
            kind: Kind::Text,
            value,
            style,
        });
    } else {
        jobs.push(Job {
            kind: Kind::Text,
            value,
            style: Style::new(),
        });
    }
}
