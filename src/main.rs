#![allow(dead_code)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

fn guess_game() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..100);

    // println!("The secret number is {}", secret_number);

    let mut guess = String::new();

    loop {
        guess.clear();

        println!();
        println!("Please input your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}

fn foo() -> i32 {
    let arr = [1, 2, 3, 4, 5];

    // Compiler error:
    // println!("Value is: {}", arr[7]);

    println!("Value is: {}", arr[4]);

    // Return value:
    42
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn test1() {
    let rect = Rectangle {
        width: 50,
        height: 100,
    };

    // Same semantics as next line
    // println!("The area of the rectangle is {}", rect.area());
    println!("The area of the rectangle is {}", (&rect).area());
    println!("The perimeter of the rectangle is {}", peri(&rect));
    println!("The Rectangle itself is {:#?}", rect);
}

fn peri(rect: &Rectangle) -> u32 {
    2 * (rect.width + rect.height)
}

fn test2() {
    let mut hash_table = HashMap::new();

    hash_table.insert(String::from("Hello"), 42);
    // println!("{}", hash_table.get("Hello"));
    // hash_table.insert(String::from("World"), 42 * 2);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
}

fn main() {
    test1();
}
