use std::{io, process::exit};

use crate::connect::handle_connect_command;

pub fn read_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let input = read_user_input("fract >> ");

        match input.as_str() {
            "exit" => exit(0),
            "worker --help" => print_help(),
            cmd if cmd.starts_with("worker connect ") => {
                handle_connect_command(cmd)?;
            }
            _ => {
                println!("Unrecognized command. Try `worker --help` for a list of commands.");
            }
        }
    }
}

fn print_help() {
    println!("\nAvailable Commands:");
    println!("  worker --help                 : Show this help menu");
    println!("  worker connect <ip:port>      : Connect to a fractal server");
}
