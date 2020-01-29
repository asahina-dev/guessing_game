extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 乱数生成
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");

    // 入力値 : 可変変数である必要がある
    let mut guess = String::new();

    // Stringでなければエラーを返す
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
