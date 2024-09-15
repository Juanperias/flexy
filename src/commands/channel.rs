use crate::config::{FLEXY_CHANNEL, FLEXY_VERSION};

pub fn get_channel() {
    println!("Flexy {} {}", FLEXY_VERSION, FLEXY_CHANNEL);
}
