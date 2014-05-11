extern crate rustic;

use rustic::sql::{Connection,SQLITE3};
//use std::io::{IoResult};

fn main() {
	match Connection::new(SQLITE3, "test.db") {
		Ok(db) => {
					match db.prepare_statement("CREATE TABLE IF NOT EXISTS t(i INTEGER PRIMARY KEY);") {
						Ok(st) => {
							for mut i in st.execute() {
								i.get_string(0);
							}
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					match db.prepare_statement("INSERT INTO t VALUES (1);") {
						Ok(st) => {
							for mut i in st.execute() {
								i.get_string(0);
							}
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					match db.prepare_statement("INSERT INTO t VALUES (2);") {
						Ok(st) => {
							for mut i in st.execute() {
								i.get_string(0);
							}
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					match db.prepare_statement("INSERT INTO t VALUES (3);") {
						Ok(st) => {
							for mut i in st.execute() {
								i.get_string(0);
							}
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					match db.prepare_statement("SELECT i FROM t;") {
						Ok(st) => {
							for mut i in st.execute() {
								println!("{}", i.get_string(0));
							}
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					}
		Err(e) => match e.detail {
						None => (),
						Some(s) => println!("{}", s)
					}
	}
}