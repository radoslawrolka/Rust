use std::io;
use rand::{thread_rng, Rng};

fn main() {
    println!("Length of password: ");
    let length = get_input_num();

    let mut password = String::new();
    for _ in 0..length {
        password.push(random_char());
    }

    println!("Random password: {}", password);
}

fn get_input_num() -> i32 {
    let mut user_input = String::new();
    let _ = io::stdin().read_line(&mut user_input);
    let num: i32 = user_input.trim().parse::<i32>().unwrap();
    num
}

fn random_char() -> char {
    let mut rng = thread_rng();
    let rand_number = rng.gen_range(33..126);
    rand_number as u8 as char
}