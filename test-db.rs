extern crate rustic;

use rustic::sql::{Connection,SQLITE3};
//use std::io::{IoResult};

fn main() {
	match Connection::new(SQLITE3, "test-db.db") {
		Ok(db) => {
					match db.prepare_statement("CREATE TABLE t(i INTEGER PRIMARY KEY);") {
						Ok(st) => {
							for i in st.execute_query() {
								match i {
									Ok(mut s)  => println!("{}", s.get_string(0)),
									Err(e) => match e.detail {
										Some(s) => println!("{}", s),
										None => ()
									}
								}
							}
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					match db.prepare_statement("INSERT INTO t VALUES (1);") {
						Ok(st) => {
							for i in st.execute_query() {
								match i {
									Ok(mut s)  => println!("{}", s.get_string(0)),
									Err(e) => match e.detail {
										Some(s) => println!("{}", s),
										None => ()
									}
								}
							}
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					match db.prepare_statement("INSERT INTO t VALUES (2);") {
						Ok(st) => {
							for i in st.execute_query() {
								match i {
									Ok(mut s)  => println!("{}", s.get_string(0)),
									Err(e) => match e.detail {
										Some(s) => println!("{}", s),
										None => ()
									}
								}
							}
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					match db.prepare_statement("INSERT INTO t VALUES (3);") {
						Ok(st) => {
							for i in st.execute_query() {
								match i {
									Ok(mut s)  => println!("{}", s.get_string(0)),
									Err(e) => match e.detail {
										Some(s) => println!("{}", s),
										None => ()
									}
								}
							}
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					match db.prepare_statement("SELECT i FROM t;") {
						Ok(st) => {
							for i in st.execute_query() {
								match i {
									Ok(mut s)  => println!("{}", s.get_string(0)),
									Err(e) => match e.detail {
										Some(s) => println!("{}", s),
										None => ()
									}
								}
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