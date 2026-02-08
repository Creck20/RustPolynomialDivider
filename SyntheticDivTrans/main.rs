// Sythentic division translation from python to rust
// Plus basic rust notes

// Standard io library
use std::io;

// Global Struct (redundant, but for the sake of understanding)
struct Division {
    dividend: Vec<i32>,
    divisor: i32,
    quotient: Vec<i32>,
}

// Program entry point, similar to c and c++
fn main()
{
    let mut Div = Division {
        dividend: Vec::new(),
        divisor: 0, 
        quotient: Vec::new(),
    }; 
    
    // Populate divisor and quotient
    // Mut makes variable mutable
    let mut finished: bool = false;
    let mut selection: String = String::new(); // User selection
    let mut value: i32 = 0; // Translate user input into dividend/divisor value
    let mut choice: i32 = 0; // Translate user selection to int

    while !finished
    {
        // Pass dividend by reference to avoid losing ownership
        display_populate(&Div.dividend, Div.divisor);
        println!("Selection: ");

        // User input: 
        // &mut refference to selection variable (which is mutable)
        // .expect is a built in error handling message
        // will have built in newline character
        selection.clear(); // Reading a new line into the string does not overwrite the string
        io::stdin().read_line(&mut selection).expect("Failed to read line");
        choice = selection.trim().parse().expect("Enter a number");

        // Match case similar to switch in C
        match choice 
        {
            1 => {
                println!("Value: ");
                selection.clear();
                io::stdin().read_line(&mut selection).expect("Failed to read line");
                value = selection.trim().parse().expect("Enter a number");
                Div.dividend.push(value);
            }
        
            2 => {
                println!("Value: ");
                selection.clear();
                io::stdin().read_line(&mut selection).expect("Failed to read line");
                value = selection.trim().parse().expect("Enter a number");
                Div.divisor = -value;
            }

            3 => {
                finished = true;
            }

            _ => { // Default case
                println!("Please enter a number between 1 and 3");
            }
        }
    }

    // Add highest degree term to solution everytime
    Div.quotient.push(Div.dividend[0]);

    // Index 1 -> last index of dividend
    for i in 1..Div.dividend.len()
    {
        Div.quotient.push(Div.divisor * Div.quotient[i-1] + Div.dividend[i]);
    }

    // Print Quoteint: 
    println!("Quotient: {:?}", Div.quotient);
}   

// Note that this function is implicitly void as it has no
// -> return type after the () parameters input 
// Note that the dividend must be passed as a 
fn display_populate(dividend: &Vec<i32>, divisor: i32) 
// Parameters inputed as name:datatype
{
    // Note: println! is a macro. Similar to preprocessor macros in C,
    // it replaces code written before compliation time. 
    println!("Please select an option:"); // \n automatically added
    println!("    1: Populate dividend");
    println!("    2: Populate divisor");
    println!("Dividend: {:?}", dividend); // {:?} Is the debug format principle
    println!("Divisor: {}", divisor);
}   

/*
Test using: 
Dividend: [2, -5, -1, 3]
Divisor: 3 -- -3
Quotient: [2, -11, 32, -93]
*/

