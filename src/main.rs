extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // 乱数生成
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    println!("Please input your guess. (1 ~ 100)");

    // 入力値 : 可変変数である必要がある
    let mut guess = String::new();

    // Stringでなければエラーを返す
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // `u32`でシャドーイングする
    let guess: u32 = guess.trim().parse().expect("Please type a number >_<");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small..."),
        Ordering::Greater => println!("Too big..."),
        Ordering::Equal => println!("You win!!"),
    }
}
