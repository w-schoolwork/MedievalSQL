use std::io::{BufReader, Cursor};

use unzip::Unzipper;

fn main() {
	let body = reqwest::blocking::get("http://aerolab.github.io/blockrain.js/dist/blockrain.zip")
		.unwrap()
		.bytes()
		.unwrap();
	let unzipper = Unzipper::new(Cursor::new(body), "./static/blockrain");
	unzipper.unzip().unwrap();
}
