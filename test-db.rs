extern crate rustic;

use rustic::sql::{Connection,SQLite3};
//use std::io::{IoResult};

fn main() {
	match Connection::new(SQLite3, "test-db.db") {
		Ok(db) => {
					match db.prepare_statement("CREATE TABLE t(i INTEGER PRIMARY KEY, f REAL, t TEXT, b BLOB);") {
					//match db.prepare_statement("CREATE TABLE t(i INTEGER PRIMARY KEY, f REAL, t TEXT);") {
						Ok(mut st) => {
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
						Ok(mut st) => {
							match st.set_long(1,10) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_double(2,10.1) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_string(3, "one") { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_blob(4, vec![1, 2, 3]) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.execute() { None=> (), Some(e) => 	match e.detail {Some(s) => println!("{}", s), None => ()} }
							match st.set_long(1,15) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_double(2,15.1) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_null(3) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_blob(4, vec![1, 2, 3]) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.execute() { None=> (), Some(e) => 	match e.detail {Some(s) => println!("{}", s), None => ()} }
							match st.set_long(1,20) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_double(2,20.2) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_string(3, "two") { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_blob(4, vec![1, 2, 3]) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.execute() { None=> (), Some(e) => 	match e.detail {Some(s) => println!("{}", s), None => ()} }
							match st.set_long(1,25) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_double(2,25.1) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_null(3) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_blob(4, vec![1, 2, 3]) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.execute() { None=> (), Some(e) => 	match e.detail {Some(s) => println!("{}", s), None => ()} }
							match st.set_long(1,30) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_double(2,30.3) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_string(3, "three") { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_blob(4, vec![1, 2, 3]) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.execute() { None=> (), Some(e) => 	match e.detail {Some(s) => println!("{}", s), None => ()} }
						},
						Err(e) => match e.detail {
							None => (),
							Some(s) => println!("{}", s)
						}
					}
					match db.prepare_statement("SELECT i,f,t,b FROM t where t like ?;") {
						Ok(mut st) => {
							st.set_string(1, "%o%");
							for i in st.execute_query() {
								match i {
									Ok(s)  => println!("{}:{}:{}:{}",	s.get_long(0), s.get_double(1),
																		s.get_string(2), s.get_blob(3) ),
									Err(e) => match e.detail {
										Some(s) => println!("{}", s),
										None => ()
									}
								}
							}
							st.set_string(1, "%e%");
							println!("----------------------------------------------------");
							for i in st.execute_query() {
								match i {
									Ok(s)  => println!("{}:{}:{}:{}", 	s.get_long(0), s.get_double(1),
																		s.get_string(2), s.get_blob(3) ),
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