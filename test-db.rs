extern crate rustic;

use rustic::sql::{Connection,SQLITE3};
//use std::io::{IoResult};

fn main() {
	match Connection::new(SQLITE3, "test-db.db") {
		Ok(db) => {
					match db.prepare_statement("CREATE TABLE t(i32 INTEGER PRIMARY KEY, i64 INTEGER, f32 REAL, f64 REAL);") {
						Ok(st) => {
							match st.execute() {
									None    => (),
									Some(e) => 	match e.detail {
													Some(s) => println!("{}", s),
													None => ()
									}
							}				
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					match db.prepare_statement("INSERT INTO t VALUES (?,?,?,?);") {
						Ok(st) => {
							match st.set_int(1,1) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()} }
							match st.set_long(2,1) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_float(3,1.0) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_double(4,1.0) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.execute() { None=> (), Some(e) => 	match e.detail {Some(s) => println!("{}", s), None => ()} }
							match st.set_int(1,2) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()} }
							match st.set_long(2,2) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_float(3,2.0) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_double(4,2.0) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.execute() { None=> (), Some(e) => 	match e.detail {Some(s) => println!("{}", s), None => ()} }
							match st.set_int(1,3) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()} }
							match st.set_long(2,3) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_float(3,3.0) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_double(4,3.0) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.execute() { None=> (), Some(e) => 	match e.detail {Some(s) => println!("{}", s), None => ()} }
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					match db.prepare_statement("SELECT * FROM t;") {
						Ok(st) => {
							for i in st.execute_query() {
								match i {
									Ok(mut s)  => println!("{}:{}:{}:{}", s.get_int(0), s.get_long(1),
																		s.get_double(2), s.get_float(3), ),
									Err(e) => match e.detail {
										Some(s) => println!("{}", s),
										None => ()
									}
								}
							}
							println!("----------------------------------------------------");
							for i in st.execute_query() {
								match i {
									Ok(mut s)  => println!("{}:{}:{}:{}", s.get_int(0), s.get_long(1),
																		s.get_double(2), s.get_float(3), ),
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