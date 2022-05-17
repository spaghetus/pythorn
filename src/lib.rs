use std::io::{BufRead, Write};

pub fn pythorn<I: BufRead, O: Write, S: Into<String>>(input: &mut I, output: &mut O, indent: S) {
	let mut indent_level = 0;
	let indent: String = indent.into();
	let indent = indent.trim();
	for line in input.lines().flatten() {
		for _ in 0..indent_level {
			output.write(indent.as_bytes()).unwrap();
		}
		if line.ends_with(':') {
			indent_level += 1;
		} else if line.ends_with("pass") {
			indent_level -= 1;
			if line == "pass" {
				// Strip passes because I have no idea whether they are actually valid in every context
				return;
			}
		}
		output.write(line.as_bytes()).unwrap();
		output.write(&[b'\n']).unwrap();
	}
}
