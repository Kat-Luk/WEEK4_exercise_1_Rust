use std::thread;
use std::io;
use std::io::Write;

fn main() {
    println!("$ Commands: read, prank, help, end.");
    loop {
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");
        thread::spawn(move || {
            match user_input.trim() {
                "read" => read_file(),
                "prank" => prank_user(),
                "end" => std::process::exit(0),
                _ => println!("Invalid command. Try again.")
        }
        });
    }
}

fn read_file() {
    match fs::read_to_string("read.txt") {
        Ok(content) => print!("{}", content),
        Err(_) => println!("$ eww"),
    }

}

fn prank_user() {
    println!("$ You have recived an email.");
}
