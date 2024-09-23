pub enum Channel {
    Unstable,
    Stable,
}

impl Channel {
    pub fn to_str(&self) -> &'static str {
        match self {
            Channel::Unstable => "Unstable",
            Channel::Stable => "Stable",
        }
    }
}

pub const FLEXY_CHANNEL: Channel = Channel::Unstable;
pub const FLEXY_VERSION: &str = "0.1";
