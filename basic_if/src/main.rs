/* 
Figure out the amount they need to pay based on choices
S = 15 / M = 20 / L = 25 / Spepperoni = +2 / M+Lpepperoni = +3 / Any extra cheese = +1
 */


fn main() {
    println!("Welcome to rust pizzia deliveries!");
    println!("What size pizzia do you want? S, M, or L: ");
    
    // Get user input for pizzia size
    let mut size = String::new();
        std::io::stdin()
            .read_line(&mut size)
            .expect("Failed to read line");

            // Determine the base price based on size
        if size.trim_end() == "S" {
            println!("You have selected a small pizzia. That will be $15.");
        } else if size.trim_end() == "M" {
            println!("You have selected a medium pizzia. That will be $20.");
        } else if size.trim_end() == "L" {
            println!("You have selected a large pizzia. That will be $25.");
        } else {
            println!("Please specify L, M, or S.");
        }
    
    // Get user input for pepperoni addition
    let mut pepperoni = String::new();
        println!("Do you want pepperoni? Y or N: ");
        std::io::stdin()
            .read_line(&mut pepperoni)
            .expect("Failed to read line");

            // Determine the price addition based on pepperoni choice and size
        if pepperoni.trim_end() == "Y" {
            if size.trim_end() == "S" {
                println!("You have added pepperoni to your small pizzia. That will be an extra $2.");
            } else if size.trim_end() == "M" || size.trim_end() == "L" {
                println!("You have added pepperoni to your medium or large pizzia. That will be an extra $3.");
            }
        } else if pepperoni.trim_end() == "N" {
            println!("You have not added pepperoni to your pizzia.");
        } else {
            println!("Please specify Y or N.");
        }

    // Get user input for extra cheese addition
    let mut extra_cheese = String::new();
        println!("Do you want extra cheese? Y or N: ");
        std::io::stdin()
            .read_line(&mut extra_cheese)
            .expect("Failed to read line");

        // Determine the price addition based on extra cheese choice
        if extra_cheese.trim_end() == "Y" {
            println!("You have added extra cheese to your pizzia. That will be an extra $1.");
        } else if extra_cheese.trim_end() == "N" {
            println!("You have not added extra cheese to your pizzia.");
        } else {
            println!("Please specify Y or N.");
        }
}

/*print("Welcome to Python Pizzia Deliveries!")
size = input("What size pissia do you want? S, M, or L: ")
pepperoni = input("Do you want pepperoni? Y or N: ")
extra_cheese = input("Do you want extra cheese? Y or N: ")

Figuure out the amount they need to pay based on choices
S = 15 / M = 20 / L = 25 / Spepperoni = +2 / M+Lpepperoni = +3 / Any extra cheese = +1

Bill = 0

if size == "S":
    Bill += 15
elif size == "M":
    Bill += 20
elif size == "L":
    Bill += 25
else:
    print("Please specify L, M, or S.")

if pepperoni == "Y":
    if size == "S":
        Bill += 2
    else:
        Bill += 3

if extra_cheese == "Y":
    Bill += 1

print(f"Your total price is ${Bill}") */