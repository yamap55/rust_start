use std::io;
// refs https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/guessing-game.html
fn main() {
    println!("");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed");

    println!("you guessed: {}", guess);
}
