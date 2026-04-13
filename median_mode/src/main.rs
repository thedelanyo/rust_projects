// Program that calculates the median and mode of a list of numbers.
// Constraints: maximum 15 values, must be an odd count.

use std::{collections::HashMap as Map, io};

fn main() {
    let total: usize = loop {
        let mut input = String::new();

        println!("Enter total number of values (max 15, must be odd): ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= 15 && num % 2 != 0 => break num,
            Ok(num) if num > 15 => println!("Number must not be greater than 15"),
            Ok(num) if num % 2 == 0 => print!("Number must be odd."),
            _ => print!("Please enter a valid number"),
        }
    };

    let mut numbers: Vec<i32> = Vec::new();

    for index in 1..=total {
        let mut input = String::new();

        println!("Enter a number {}: ", index);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num: i32 = input.trim().parse().expect("Please enter a valid number");

        numbers.push(num);
    }

    numbers.sort();

    let median = numbers[total / 2];
    let mut frequency = Map::new();

    for num in &numbers {
        *frequency.entry(num).or_insert(0) += 1;
    }

    let mut mode = numbers[0];
    let mut max_count = 0;

    for (num, count) in &frequency {
        if *count > max_count {
            max_count = *count;
            mode = **num;
        }
    }

    println!("Sorted Numbers: {:?}", numbers);
    println!("Median: {}", median);
    println!("Mode: {}", mode)
}
