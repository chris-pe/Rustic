extern crate rustic;

use rustic::sql::Connection;
use rustic::sql::DbType::SQLite3;

fn main() {
	match Connection::new(SQLite3, "test-db.db") {
		Ok(db) => {
					match db.prepare_statement("CREATE TABLE t(i INTEGER PRIMARY KEY, f REAL, t TEXT, b BLOB);") {
						Ok(mut st) => {
							match st.execute() {
									None    => (),
									Some(e) => 	println!("{}", e)
							}				
						},
						Err(e) =>  	println!("{}", e)
						
					}
					match db.prepare_statement("INSERT INTO t VALUES (?,?,?,?);") {
						Ok(mut st) => {
							match st.set_long(1,10) { None=>(), Some(e) => println!("{}", e) }
							match st.set_double(2,10.1) { None=>(), Some(e) => println!("{}", e) }
							match st.set_string(3, "one") { None=>(), Some(e) => println!("{}", e) }
							match st.set_blob(4, &[1, 2, 3]) { None=>(), Some(e) => println!("{}", e) }
							match st.execute() { None=> (), Some(e) => 	println!("{}", e) }
							match st.set_long(1,15) { None=>(), Some(e) => println!("{}", e) }
							match st.set_double(2,15.1) { None=>(), Some(e) => println!("{}", e) }
							match st.set_null(3) { None=>(), Some(e) => println!("{}", e) }
							match st.set_blob(4, &[4, 5, 6]) { None=>(), Some(e) => println!("{}", e) }
							match st.execute() { None=> (), Some(e) => 	println!("{}", e) }
							match st.set_long(1,20) { None=>(), Some(e) => println!("{}", e) }
							match st.set_double(2,20.2) { None=>(), Some(e) => println!("{}", e) }
							match st.set_string(3, "two") { None=>(), Some(e) => println!("{}", e) }
							match st.set_blob(4, &[7, 8, 9]) { None=>(), Some(e) => println!("{}", e) }
							match st.execute() { None=> (), Some(e) => 	println!("{}", e) }
							match st.set_long(1,25) { None=>(), Some(e) => println!("{}", e) }
							match st.set_double(2,25.1) { None=>(), Some(e) => println!("{}", e) }
							match st.set_null(3) { None=>(), Some(e) => println!("{}", e) }
							match st.set_blob(4, &[10, 11, 12]) { None=>(), Some(e) => println!("{}", e) }
							match st.execute() { None=> (), Some(e) => 	println!("{}", e) }
							match st.set_long(1,30) { None=>(), Some(e) => println!("{}", e) }
							match st.set_double(2,30.3) { None=>(), Some(e) => println!("{}", e) }
							match st.set_string(3, "three") { None=>(), Some(e) => println!("{}", e) }
							match st.set_blob(4, &[13, 14, 15]) { None=>(), Some(e) => println!("{}", e) }
							match st.execute() { None=> (), Some(e) => println!("{}", e) }
						},
						Err(e) => println!("{}", e)
					}
					match db.prepare_statement("SELECT i,f,t,b FROM t where t like ?;") {
						Ok(mut st) => {
							st.set_string(1, "%o%");
							for i in st.execute_query() {
								match i {
									Ok(s)  => println!("{}:{}:{}:{:?}",	s.get_long(0), s.get_double(1),
																		s.get_string(2), s.get_blob(3) ),
									Err(e) => println!("{}", e)
								}
							}
							st.set_string(1, "%e%");
							println!("----------------------------------------------------");
							for i in st.execute_query() {
								match i {
									Ok(s)  => println!("{}:{}:{}:{:?}", 	s.get_long(0), s.get_double(1),
																		s.get_string(2), s.get_blob(3) ),
									Err(e) => println!("{}", e)
								}
							}
							st.set_string(1, "%");
							println!("----------------------------------------------------");
							for i in st.execute_query() {
								match i {
									Ok(s)  => println!("{}:{}:{}:{:?}", 	s.get_long(0), s.get_double(1),
																		s.get_string(2), s.get_blob(3) ),
									Err(e) => println!("{}", e)
								}
							}
						},
						Err(e) => println!("{}", e)
					}
					match db.prepare_statement("SELECT i,f,t,b FROM t where t is null;") {
						Ok(mut st) => {
							println!("----------------------------------------------------");
							for i in st.execute_query() {
								match i {
									Ok(s)  => println!("{}:{}:{:}:{:?}", 	s.get_long(0), s.get_double(1),
																		String::from_utf8(s.get_blob(2)).unwrap(), s.get_blob(3) ),
									Err(e) => println!("{}", e)
								}
							}
						},
						Err(e) => println!("{}", e)
					}
					}
		Err(e) => println!("{}", e)
	}
}