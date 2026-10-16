use std::fs;
use std::io::{self, Read, Write};

fn main() {
	let mut notes: Vec<String> = Vec::new();
	println!();
	println!("Welcome to your Notepad!");

	println!();
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
			"remove" => {
				if notes.is_empty() {
					println!("No notes to remove.");
					continue;
				}
				print!("Enter the note number you want to remove: ");
				io::stdout().flush().unwrap();

				let mut num_input: String = String::new();
				io::stdin().read_line(&mut num_input).unwrap();
				let num_note: usize = num_input.trim().parse().unwrap();

				if num_note > 0 && num_note <= notes.len() {
					let removed = notes.remove(num_note - 1);
					println!("Removed: \"{removed}\"");
				} else {
					println!("Invalid note number!");
				}
			}
			"list" => {
				if notes.is_empty() {
					println!("No notes added to list.");
					continue;
				}

				println!();
				println!("====== NOTES ======");

				for (idx, note) in notes.iter().enumerate() {
					println!("{}. {}", idx + 1, note);
				}

				println!();
			}
			_ => {
				println!("Not a valid command.");
			}
		}
	}
}
