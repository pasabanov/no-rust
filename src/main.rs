//! # main.rs
//!
//! Entry point of the "no" program.
//!
//! The "no" program serves as the opposite of the "yes" program, continuously outputting "n" or all arguments passed to it separated by spaces.
//!
//! # Author
//!
//! Petr Alexandrovich Sabanov
//!
//! # Copyright
//!
//! [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.html).

use std::env::args;
use std::io::{self, Write};

const HELP_MESSAGE: &str =
"Usage: no [STRING]...
Repeatedly output a line with all specified STRING(s), or 'n'.

Options:
  --help     display this help message and exit.
  --version  display version information and exit.";

const VERSION_MESSAGE: &str = "no (Rust) 1.0.0
Copyright (C) 2024 Petr Alexandrovich Sabanov
License GPLv3.0: GNU General Public License v3.0 <https://www.gnu.org/licenses/gpl-3.0.html>";

const INVALID_OPTION_MESSAGE: &str =
"no: invalid option -- '{}'
Try 'no --help' for more information.";

const UNRECOGNIZED_OPTION_MESSAGE: &str =
"no: unrecognized option '{}'
Try 'no --help' for more information.";

fn main() {

	let args: Vec<String> = args().collect();

	let mut message = String::new();

	let mut parse_args = true;

	for arg in &args[1..] {
		if arg == "--" {
			parse_args = false;
			continue;
		}

		if parse_args && arg.starts_with("-") {
			if arg.starts_with("--") {
				if arg == "--help" {
					println!("{}", HELP_MESSAGE);
				} else if arg == "--version" {
					println!("{}", VERSION_MESSAGE);
				} else {
					eprintln!("{}", UNRECOGNIZED_OPTION_MESSAGE.replace("{}", arg));
				}
			} else {
				eprintln!("{}", INVALID_OPTION_MESSAGE.replace("{}", arg));
			}
			return;
		}

		if !message.is_empty() {
			message.push(' ');
		}
		message.push_str(arg);
	}

	if message.is_empty() {
		message = "n\n".to_string();
	} else {
		message.push('\n');
	}

	let mut out = io::stdout();
	loop {
		if let Err(_) = out.write_all(message.as_bytes()) {
			break;
		}
		if let Err(_) = out.flush() {
			break;
		}
	}
}