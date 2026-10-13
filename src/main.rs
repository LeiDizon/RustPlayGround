use rust_playground::{concept_map, countdown, even_numbers_up_to, sum_first_n};

fn main() {
    println!("Rust Playground");
    println!("================");
    println!("This project is meant to help you practice Rust fundamentals.");
    println!("");

    let numbers = countdown(5);
    println!("Countdown from 5: {:?}", numbers);

    let total = sum_first_n(10);
    println!("Sum of the first 10 numbers: {}", total);

    let evens = even_numbers_up_to(10);
    println!("Even numbers up to 10: {:?}", evens);

    println!("");
    println!("Concepts to practice:");
    for concept in concept_map() {
        println!("- {}", concept);
    }

    println!("");
    println!("Exercises to try:");
    println!("1. Change the countdown start value and explain what happens with the range expression.");
    println!("2. Write a for loop that prints the squares of numbers from 1 to 10.");
    println!("3. Create a Vec<i32> and append values using a loop, then print the result.");
    println!("4. Rework the even_numbers_up_to function using a while loop and compare it to the for loop version.");
    println!("5. Add a new function and document which Rust concepts it teaches.");
}
