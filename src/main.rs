use std::fs;
use std::io::{self, Read, Write};

fn main() {
	let mut notes: Vec<String> = Vec::new();
	println!();
	println!("Welcome to your Notepad!");

	println!();
	println!("Commands: [add]  [remove]  [list]  [save]  [exit]\n");

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
			"save" => {
				if notes.is_empty() {
					println!("List is empty. Consider adding a note before saving.");
					continue;
				}

				print!("Enter a name for this list: ");
				io::stdout().flush().unwrap();

				let mut file_name = String::new();
				io::stdin().read_line(&mut file_name).unwrap();
				file_name = file_name.trim().to_string();

				let file = format!("{file_name}.txt");
				let notes_content = notes.join("\n");

				match fs::write(&file, notes_content) {
					Ok(_) => println!("Successfully saved notes to {file}!"),
					Err(e) => println!("Failed to save file: {e}."),
				}
			}
			"exit" => {
				println!("Closing Notepad. See you next time!");
				break;
			}
			_ => {
				println!("Not a valid command.");
			}
		}
	}
}
