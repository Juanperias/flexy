use colored::Colorize;

pub fn lua_debug(message: String) {
    println!("[{}] {}", "LUA-DEBUG".blue(), message);
}

pub fn lua_error(message: String) {
    println!("[{}] {}", "LUA-ERROR".red(), message);
    std::process::exit(1);
}
