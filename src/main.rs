use std::io;
use std::io::Write;

// Write a program that prompts for two numbers. Print the sum, difference, product, and quotient of those numbers as shown in the example output:
// Example Output
//        What is the first number? 10
//        What is the second number? 5
//        10 + 5 = 15
//        10 - 5 = 5
//        10 * 5 = 50
//        10 / 5 = 2

// prompt, print
// two numbers, sum, difference, product, quotient

// Input: Two numbers
// Process: get sum, difference, product, quotient
// Output: all

fn main() -> io::Result<()> {
    let mut first_number = String::new();
    let mut second_number = String::new();

    print!("What is the first number? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut first_number)?;
    // Check if not number
    // Dont proceed
    
    print!("What is the second number? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut second_number)?;
    // Check if not number
    // Dont proceed

    let first_number_parsed: i32 = first_number.trim().parse::<i32>().unwrap();
    let second_number_parsed: i32 = second_number.trim().parse::<i32>().unwrap();

    let sum = first_number_parsed + second_number_parsed;
    let diff = first_number_parsed - second_number_parsed;
    let prod = first_number_parsed * second_number_parsed;
    let quo = first_number_parsed / second_number_parsed;

    println!("{first} + {second} = {answer}", first=first_number_parsed, second=second_number_parsed, answer=sum);
    println!("{first} - {second} = {answer}", first=first_number_parsed, second=second_number_parsed, answer=diff);
    println!("{first} * {second} = {answer}", first=first_number_parsed, second=second_number_parsed, answer=prod);
    println!("{first} / {second} = {answer}", first=first_number_parsed, second=second_number_parsed, answer=quo);

    Ok(())
}
