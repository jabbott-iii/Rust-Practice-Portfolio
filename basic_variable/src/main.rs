use std::io;
use std::io::BufRead;

fn main() {
    println!("Welcome to the band name generator.");

    println!("What city were you born in?");

    let mut city: String = String::new();
    io::stdin()
        .read_line(&mut city)
        .expect("Failed to read line");

    println!("What was the name of your first pet?");

    let mut pet: String = String::new();
    io::stdin()
        .read_line(&mut pet)
        .expect("Failed to read line");

    println!("Your band name is {}{}", city.trim_end(), pet.trim_end());

    println!("Program finished. Press Enter to exit...");
        let stdin = io::stdin();
        let _ = stdin.lock().lines().next();
}