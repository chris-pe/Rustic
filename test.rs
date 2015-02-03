extern crate rustic;

use rustic::sql::{Connection,SQLite3};

fn main() {
	match Connection::new(SQLite3, "test.db") {
		Ok(db) => 	{
					}
		Err(e) => match e.detail {
						None => (),
						Some(s) => println!("{}", s)
					}
	}
}