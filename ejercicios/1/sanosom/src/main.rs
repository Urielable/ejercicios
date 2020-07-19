use ferris_says::say;
use std::io::{stdin, stdout, BufWriter};

fn main() {
    let stdin = stdin();
    let stdout = stdout();

    println!("What do you wanna Ferris say?");

    let mut message = String::new();

    stdin.read_line(&mut message).expect("Failed to read line");

    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());

    say(message.as_bytes(), width, &mut writer).unwrap();
}
