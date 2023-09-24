
use std::io;

fn main() {
	println!("Guess the number from 1 to 10!");

	println!("Input your guess: ");

	// mut = mutable
	let mut guess = String::new();

	// read standard input stream
	io::stdin()
		.read_line(&mut guess) // reference to the mutable string
		.expect("Faileed to read line"); // exception

	println!("You have guessed: {guess}"); // place the string into the text

}
