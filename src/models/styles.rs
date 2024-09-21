// TODO

pub trait Style {
    fn from_table() -> Self;
}

pub struct TextStyle {}

impl Style for TextStyle {
    fn from_table() -> Self {
        TextStyle {}
    }
}
