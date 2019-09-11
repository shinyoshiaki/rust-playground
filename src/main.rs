extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn str2u32(s: &str) -> (bool, u32) {
    let mut error: bool = false;
    let mut res: u32 = 0;
    match s.trim().parse::<u32>() {
        Err(_) => error = true,
        Ok(n) => res = n,
    }
    return (error, res);
}

fn main() {
    let sec = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("fail");

    let (error, guess) = str2u32(&guess);

    if !error {
        println!("guess {}", guess);

        match guess.cmp(&sec) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => println!("equal"),
        }
    } else {
        println!("parse error")
    }
    println!("end")
}
