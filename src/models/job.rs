use crate::models::styles::Style;

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    Text,
}

#[derive(Debug)]
pub struct Job {
    pub kind: Kind,
    pub value: String,
    pub style: Style,
}
