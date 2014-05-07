extern crate rustic;

use std::io::{File, Open, Read};
use rustic::util::Properties;

fn main() {
	let p = Path::new("test.properties");
	let f = File::open_mode(&p, Open, Read);
	let mut props = Properties::new();
	println!("num:{}", props.load(f));
	for i in props.iter() {
		println!("{}", i);
	}
}