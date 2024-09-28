use colored::Colorize;

pub fn lua_debug(message: &str) {
    println!("[{}] {}", "LUA-DEBUG".blue(), message);
}

pub fn lua_error(message: &str) {
    println!("[{}] {}", "LUA-ERROR".red(), message);
    std::process::exit(1);
}
