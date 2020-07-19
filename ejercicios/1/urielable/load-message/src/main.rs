use ferris_says::say;
use std::io;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    println!("hey who are you?");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let message = format!("{}\n{}{}", "Peew pew pew, your died ....", guess, "Killed in action.");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
