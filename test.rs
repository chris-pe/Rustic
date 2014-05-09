extern crate rustic;

use std::io::{File, Open, Write, Read, Truncate};
use rustic::util::Properties;

fn main() {
	let pr = Path::new("test.properties");
	let fr = File::open_mode(&pr, Open, Read);
	let mut props = Properties::new();
	println!("num:{}", props.load(fr));
	let pw = Path::new("test-out.properties");
	let fw = File::open_mode(&pw, Truncate, Write);
	println!("num:{}", props.store(fw));
	
	for i in props.iter() {
		match i {
			(k,v) =>	{
							println!("'{}'='{}'", k, v);
						}
		}
	}
	
}