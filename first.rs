use std::process::Command;

fn clear() {
    Command::new("clear")
        .spawn();
}

fn main() {
    clear();
    println!("Hello, World!");
}