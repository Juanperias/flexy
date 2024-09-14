#[derive(Debug)]
pub enum JobKind {
    Text,
}

#[derive(Debug)]
pub struct Job {
    pub kind: JobKind,
    pub value: String,
}
