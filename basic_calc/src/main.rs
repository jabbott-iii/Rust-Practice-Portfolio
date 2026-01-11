std::io;
std::io::BufRead;

fn main() {
    println!("Tip Calculator");
    
    println!("What is the total bill? $");
    let mut bill = float::new(); // Variable to hold the total bill amount
    io::stdin() // Get standard input
        .read_line(&mut bill) // Read a line into the bill variable
        .expect("Failed to read line"); // Handle potential errors

    println!("What percentage would you like to give as a tip? ");
    let mut tip = float::new(); // Variable to hold the tip percentage
    io::stdin() // Get standard input
        .read_line(&mut tip) // Read a line into the tip variable
        .expect("Failed to read line"); // Handle potential errors

    let percent = tip / 100.0; // Calculate the tip percentage
    let total = percent * bill; // Calculate the total tip amount

    println!("How many people are you splitting the bill with? Including yourself. ");
    let mut friends = float::new(); // Variable to hold the number of people
    io::stdin() // Get standard input
        .read_line(&mut friends) // Read a line into the friends variable
        .expect("Failed to read line"); // Handle potential errors

    let final_amount = total / friends; // Calculate the individual share of the tip

    println!("Individual share of the tip is ${:.2}", final_amount); // Display the individual share rounded to 2 decimal places

    println!("Program finished. Press Enter to exit...");
    let stdin = io::stdin(); // Get standard input
    let _ = stdin.lock().lines().next(); // Wait for user to press Enter
}


/*print("Tip Caclulator")
bill = float(input("What is the total bill? $"))
tip = float(input("What percentage would you like to give as a tip? "))
percent = (tip / 100)
total = (percent * bill)
friends = float(input("How many people are you splitting the bill with? Including yourself. "))
final = (total / friends)
print("Individual share of the tip is $"+str(round(final, 2))) */