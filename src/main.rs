use std::thread;
use std::io;
use std::io::Write;
use std::fs;

fn main() {
    loop {
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }

        match user_input.trim() {
            "read" => read_file(),
            "prank" => prank_user(),
            "end" => std::process::exit(0),
            "help" => println!("$ Commands: read, prank, help, end."),
            _ => println!("Invalid command. Try again."),
        }
    }
}

fn read_file() {
    match fs::read_to_string("read.txt") {
        Ok(content) => print!("{}", content),
        Err(_) => println!("$ eww"),
    }

}

fn prank_user() {
    println!("$ You have received an email.");
}
