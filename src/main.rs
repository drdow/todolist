use std::io;
use std::io::Write;

fn main() {
    let commands: [&str; 3] =  [
        "add",
        "list",
        "exit"
    ];

    println!("-- Welcome to To-Do app! --");
    println!("Please, enter a command:");

    for command in commands.iter() {
        println!("  {}", command);
    }
    println!("Your command:");
    io::stdout().flush().unwrap();

    let mut command: String = String::new();
    io::stdin().read_line(&mut command)
        .expect("Failed to read command");

    match command.trim() {
        "add" => {
            println!("Adding a new entry (NOT IMPLEMENTED)");
        },
        "list" => {
            println!("Listing all available entries (NOT IMPLEMENTED)");
        },
        "exit" => {
            println!("Exiting...");
            return;
        },
        _ => {
            println!("Invalid command");
        }
    }
}
