use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // 入力値 : 可変変数である必要がある
    let mut guess = String::new();

    // Stringでなければエラーを返す
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
