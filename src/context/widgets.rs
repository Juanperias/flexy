use crate::models::job::{Job, Kind};
use crate::models::styles::Style;
use mlua::Table;

pub fn clear(jobs: &mut Vec<Job>) {
    *jobs = Vec::new();
}

pub fn text(value: String, table: Option<Table>, jobs: &mut Vec<Job>) {
    if let Some(safe_table) = table {
        let style = Style::from_table(&safe_table).expect("Error: cannot convert the style");

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
