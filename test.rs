extern crate rustic;

use rustic::sql::Connection;
use rustic::sql::DbType::SQLite3;

fn main() {
	match Connection::new(SQLite3, "test.db") {
		Ok(db) => 	{
					match db.prepare_statement("CREATE TABLE t(i INTEGER PRIMARY KEY, f REAL);") {
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
					
						match db.prepare_statement("INSERT INTO t VALUES (?,?);") {
						Ok(mut st) => {
							match st.set_long(1,10) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_double(2,10.1) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.execute() { None=> (), Some(e) => 	match e.detail {Some(s) => println!("{}", s), None => ()} }
							match st.set_long(1,15) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.set_double(2,15.1) { None=>(), Some(e) => match e.detail {Some(s) => println!("{}", s), None => ()}  }
							match st.execute() { None=> (), Some(e) => 	match e.detail {Some(s) => println!("{}", s), None => ()} }
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