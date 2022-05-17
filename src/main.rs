use std::{
	io::{BufRead, BufReader, Write},
	path::PathBuf,
};

use structopt::StructOpt;

#[macro_use]
extern crate structopt;

#[derive(Debug, StructOpt)]
#[structopt(
	name = "pythorn",
	about = "a program to make python's indentation less of a thorn in your side"
)]
pub struct Options {
	#[structopt(short, long)]
	pub spaces: Option<u8>,
	#[structopt(short, long, parse(from_os_str))]
	pub input: Option<PathBuf>,
	#[structopt(short, long, parse(from_os_str))]
	pub output: Option<PathBuf>,
}

fn main() {
	let opt = Options::from_args();

	let mut input: Box<dyn BufRead> = match opt.input {
		Some(path) => Box::new(BufReader::new(
			std::fs::File::open(path).expect("Could not open input file"),
		)),
		None => Box::new(std::io::stdin().lock()),
	};
	let mut output: Box<dyn Write> = match opt.output {
		Some(path) => Box::new(std::fs::File::create(path).expect("Could not open output file")),
		None => Box::new(std::io::stdout()),
	};

	let indent: String = match opt.spaces {
		Some(n) => [' '].repeat(n.into()).iter().collect(),
		None => vec!['\t'].iter().collect(),
	};

	pythorn::pythorn(&mut input, &mut output, &indent);
}
