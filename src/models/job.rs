#[derive(Debug)]
pub enum Kind {
    Text,
}

#[derive(Debug)]
pub struct Job {
    pub kind: Kind,
    pub value: String,
}
