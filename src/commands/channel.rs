use crate::config::{FLEXY_CHANNEL, FLEXY_VERSION};

pub fn get() {
    println!("Flexy {FLEXY_VERSION} {}", FLEXY_CHANNEL.to_str());
}
