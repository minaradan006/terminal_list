use std::fs;
use std::io::{self, Read, Write};

fn main() {
	let mut notes: Vec<String> = Vec::new();

	println!("Commands: [add]  [remove]  [list]  [save]  [delete]  [exit]\n");

	loop {
		print!("> ");
		io::stdout().flush().unwrap();

		let mut input = String::new();
		if io::stdin().read_line(&mut input).is_err() {
			println!("Failed to read input.");
			continue;
		}

		let command = input.trim();

		match command {
			"add" => {
				print!("Enter new note: ");
				io::stdout().flush().unwrap();

				let mut note = String::new();
				io::stdin().read_line(&mut note).unwrap();
				note = note.trim().to_string();

				if !note.is_empty() {
					notes.push(note);
					println!("Note added!");
				}
			}
		}
	}
}
