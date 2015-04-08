extern crate rustic;

use rustic::util::Properties;
use std::path::Path;
use std::fs::File;

fn main() {

	let pr = Path::new("test-props-in.properties");
	let fr = match File::open(pr) {
		Ok(f) => f,
		Err(e) => panic!("file error: {}", e),
	};

	let mut props = Properties::new();
	props.load(fr);
	
	for i in props.iter() {
		match i {
			(k,v) => println!("'{}'='{}'", k, v)
		}
	}

	let pw = Path::new("test-props-out.properties");
	let fw = match File::create(pw) {
		Ok(f) => f,
		Err(e) => panic!("file error: {}", e),
	};
	
	props.store(fw);

}