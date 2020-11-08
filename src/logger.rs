use colored::*;

pub fn error_and_exit(msg: &str, e: &dyn std::error::Error) -> ! {
    eprintln!("{} {} [{}]!", "ERROR:".red(), msg.red(), e);
    std::process::exit(1);
}

pub fn _warn(msg: &str, e: &dyn std::error::Error) {
    println!("{} {} [{}]!", "WARNING:".yellow(), msg.yellow(), e);
}
