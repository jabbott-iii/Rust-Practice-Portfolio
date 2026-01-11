use std::io;
use std::io::BufRead;

fn main() {
    println!("Welcome to the band name generator.");

    println!("What city were you born in?");

    let mut city: String = String::new(); // String type to hold user input
    io::stdin() // Get standard input
        .read_line(&mut city) // Read a line into the city variable
        .expect("Failed to read line"); // Handle potential errors

    println!("What was the name of your first pet?");

    let mut pet: String = String::new();
    io::stdin()
        .read_line(&mut pet)
        .expect("Failed to read line");

    println!("Your band name is {}{}", city.trim_end(), pet.trim_end()); // Concatenate and display the band name


    println!("Program finished. Press Enter to exit...");
        let stdin = io::stdin(); // Get standard input
        let _ = stdin.lock().lines().next(); // Wait for user to press Enter
}