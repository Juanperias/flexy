use crate::models::styles::Style;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    Text,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub kind: Kind,
    pub value: String,
    pub style: Style,
}
