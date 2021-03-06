extern crate clml_rs;

use std::fs;
use std::path::{ Path, PathBuf };

use clml_rs::{ clml };

type BuildList = Vec<(PathBuf, PathBuf)>;

fn get_buildlist(base: &Path) -> BuildList {
	let mut to_return: BuildList = Vec::new();
	let buildlist_string = fs::read_to_string(base.join("buildlist")).expect("Failed to read buildlist file!");
	let buildlist_lines = {
		let split: Vec<&str> = buildlist_string.split("\n").collect();
		let mut to_return = Vec::new();
		for line in split.iter() {
			if !line.starts_with("#") {
				to_return.push(line.clone());
			}
		}
		to_return
	};
	for line in buildlist_lines.iter() {
		let split: Vec<&str> = line.split(" -> ").collect();
		if split.len() != 2 { panic!("Expected only one ... -> ... statement per line!"); }
		to_return.push((base.join(split[0]), base.join(split[1])));
	}
	to_return
}

fn progress(min: usize, max: usize) -> String {
	let mut bar = String::from(" [");
	let complete = ((min as f32 / max as f32) * 57.0).floor() as usize;
	if complete >= 1 {
		bar = format!("{}{}",
			bar,
			String::from("=").repeat(complete - 1));
	}
	bar = format!("{}>", bar);
	let remaining = 57 - complete;
	if complete >= 1 {
		bar = format!("{}{}]",
			bar,
			String::from(" ").repeat(remaining));
	} else {
		bar = format!("{}{}]",
			bar,
			String::from(" ").repeat(remaining - 1));
	}
	format!("{bar} {min:03}/{max:03}: ",
		bar = bar,
		min = min,
		max = max)
}

fn main() {
	{
		let input = fs::read_to_string("./src/assets/help.clml").expect("Failed to read the file \"./src/assets/help.clml\"!");
		let output = clml(&input);
		fs::write("./src/assets/.help.clml", output);
	}
	let base = Path::new("./src/assets/ascii_art/");
	let buildlist = get_buildlist(&base);
	let len = buildlist.len();
	for (i, target) in buildlist.iter().enumerate() {
		println!("\u{001b}[1A\r\u{001b}[K    \u{001b}[1m\u{001b}[36mBuilding\u{001b}[0m{}ASCII art",
			progress(i, len));
		let input = fs::read_to_string(&target.0).expect(&format!("Failed to read the file \"{:?}\"!", &target.0));
		let output = clml(&input);
		fs::write(&target.1, &output).expect(&format!("Failed to write to the file \"{:?}\"!", &target.1));
	}
	println!("\u{001b}[1A\r\u{001b}[K    \u{001b}[1m\u{001b}[32mFinished\u{001b}[0m ASCII art");
}
