use std::str::from_utf8;
use std::ffi::{CString,CStr};
use std::vec::Vec;
use std::io::{Error, ErrorKind, Result};
use std::ptr::null;
use sql::DbType;
use std::slice;

#[link(name = "sqlite3")]
extern {
    fn sqlite3_open(filename : *const i8, ppDb : *const*const i8) -> i32;
	fn sqlite3_close_v2(pDb : *const i8) -> i32;
	fn sqlite3_errmsg(pDb : *const i8) -> *const i8;
	fn sqlite3_errstr(erno : i32) -> *const i8;
	fn sqlite3_prepare_v2(pDb : *const i8, sql : *const i8, nByte : i32, ppStmt : *const*const i8, pzTail : *const*const i8) -> i32;
	fn sqlite3_step(pStmt : *const i8) -> i32;
	fn sqlite3_changes(pStmt : *const i8) -> i32;
	fn sqlite3_column_int(pStmt : *const i8, iCol : i32) -> i32;
	fn sqlite3_column_int64(pStmt : *const i8, iCol : i32) -> i64;
	fn sqlite3_column_double(pStmt : *const i8, iCol : i32) -> f64;
	fn sqlite3_column_text(pStmt : *const i8, iCol : i32) -> *const u8;
	fn sqlite3_column_blob(pStmt : *const i8, iCol : i32) -> *const u8;
	fn sqlite3_column_bytes(pStmt : *const i8, iCol : i32) -> i32;
	fn sqlite3_bind_int(pStmt : *const i8, iCol : i32, value : i32) -> i32;
	fn sqlite3_bind_int64(pStmt : *const i8, iCol : i32, value : i64) -> i32;
	fn sqlite3_bind_double(pStmt : *const i8, iCol : i32, value : f64) -> i32;
	fn sqlite3_bind_text(pStmt : *const i8, iCol : i32, value : *const i8, n : i32, f: *const extern fn(*const i8)) -> i32;
	fn sqlite3_bind_null(pStmt : *const i8, iCol : i32) -> i32;
	fn sqlite3_bind_blob(pStmt : *const i8, iCol : i32, value : *const i8, n : i32, f: *const extern fn(*const i8)) -> i32;
	fn sqlite3_reset(pStmt : *const i8) -> i32;
	//fn sqlite3_finalize(pStmt : *i8) -> i32;
}

///Connection permits to connect to supported databases.
pub struct Connection {
	db_type : DbType,
	p_db : *const i8
}

///Statement is used for executing SQL instructions and returning results.
///
///In the SQL Statement, ? character is replaced by a parameter using a set_* method.
pub struct Statement<'a> {
	p_con  : &'a Connection,
	p_stmt : *const i8,
	exec  : bool
}

///Cursor is used for browsing a database query result.
pub struct Cursor<'a: 'b, 'b> {
	p_stmt : &'b Statement<'a>,
	error : bool
}

impl<'a> Statement<'a> {

	///Execute the SQL query and returns the result in an iterable Cursor.
	pub fn execute_query<'b>(&'b mut self) -> Cursor<'a, 'b> {
		match self.p_con.db_type {
		DbType::SQLite3 => {
		if self.exec { unsafe { sqlite3_reset(self.p_stmt) }; } else { self.exec=true; }
		Cursor { p_stmt : self, error : false }
		}
		}
	}

	///Execute the SQL statement and returns None if succeeds or an Io Error.
	pub fn execute(&mut self) -> Option<Error> {
		match self.p_con.db_type {
		DbType::SQLite3 => {
		if self.exec { unsafe { sqlite3_reset(self.p_stmt) }; } else { self.exec=true; }
		match unsafe { sqlite3_step(self.p_stmt) } {
			100 | 101 => None,
			err => Some (Error::new(ErrorKind::Other, get_error(self.p_con.p_db, err).as_ref())) }
		}
		}
	}

	///Execute the SQL INSERT, UPDATE or DELETE statement and returns the number of affected rows.
	///Returns 0 for SQL statement that returns nothing. Returns an Io Error if fails.
	pub fn execute_update(&mut self) -> Result<i32> {
		match self.p_con.db_type {
		DbType::SQLite3 => { 
		if self.exec { unsafe { sqlite3_reset(self.p_stmt) }; } else { self.exec=true; }
		match unsafe { sqlite3_step(self.p_stmt) } {
			100 | 101 => Ok(unsafe { sqlite3_changes(self.p_stmt) }),
			err => Err(Error::new(ErrorKind::Other, get_error(self.p_con.p_db, err).as_ref())) }
		}
		}
	}
	
	///Replace in the SQL Statement the '?' parameter by an int. The leftmost parameter has an index of 1.
	pub fn set_int(&mut self, param_index : i32, value : i32) -> Option<Error> {
		match self.p_con.db_type {
		DbType::SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.p_stmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_int(self.p_stmt, param_index, value) } {
				0 => None,
				n => Some(Error::new(ErrorKind::Other, get_error(self.p_con.p_db, n).as_ref())) }
			}
		}
	}
	
	///Replace in the SQL Statement the '?' parameter by an i64. The leftmost parameter has an index of 1.
	pub fn set_long(&mut self, param_index : i32, value : i64) -> Option<Error> {
		match self.p_con.db_type {
		DbType::SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.p_stmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_int64(self.p_stmt, param_index as i32, value) } {
				0 => None,
				n => Some(Error::new(ErrorKind::Other, get_error(self.p_con.p_db, n).as_ref())) }
		}
		}
	}

	///Replace in the SQL Statement the '?' parameter by an f32. The leftmost parameter has an index of 1.
	pub fn set_float(&mut self, param_index : i32, value : f32) -> Option<Error> {
		match self.p_con.db_type {
		DbType::SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.p_stmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_double(self.p_stmt, param_index as i32, value as f64) } {
				0 => None,
				n => Some(Error::new(ErrorKind::Other, get_error(self.p_con.p_db, n).as_ref())) }
		}
		}
	}

	///Replace in the SQL Statement the '?' parameter by a double. The leftmost parameter has an index of 1.
	pub fn set_double(&mut self, param_index : i32, value : f64) -> Option<Error> {
		match self.p_con.db_type {
		DbType::SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.p_stmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_double(self.p_stmt, param_index as i32, value) } {
				0 => None,
				n => Some(Error::new(ErrorKind::Other, get_error(self.p_con.p_db, n).as_ref())) }
		}
		}
	}

	///Replace in the SQL Statement the '?' parameter by an &str. The leftmost parameter has an index of 1.
	pub fn set_string(&mut self, param_index : i32, value : &str) -> Option<Error> {
		match self.p_con.db_type {
		DbType::SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.p_stmt) }; self.exec=false; }
			match CString::new(value) {
				Ok(s)	=> match unsafe { sqlite3_bind_text(self.p_stmt, param_index as i32, s.as_ptr(), -1i32, !0 as *const extern fn(*const i8)) } {
								0 => None,
								n => Some(Error::new(ErrorKind::Other, get_error(self.p_con.p_db, n as i32).as_ref())) },
				Err(e)	=> Some(Error::new(ErrorKind::Other, e))
			}
		}
		}
	}
	
	///Replace in the SQL Statement the '?' parameter by an &[u8]. The leftmost parameter has an index of 1.
	pub fn set_blob(&mut self, param_index : i32, value : &[u8]) -> Option<Error> {
		match self.p_con.db_type {
		DbType::SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.p_stmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_blob(self.p_stmt, param_index as i32, value.as_ptr() as *const i8,
												value.len() as i32, !0 as *const extern fn(*const i8)) } {
				0 => None,
				n => Some(Error::new(ErrorKind::Other, get_error(self.p_con.p_db, n).as_ref())) }
		}
		}
	}
	
	///Replace in the SQL Statement the '?' parameter by an SQL NULL. The leftmost parameter has an index of 1.
	pub fn set_null(&mut self, param_index : i32) -> Option<Error> {
		match self.p_con.db_type {
		DbType::SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.p_stmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_null(self.p_stmt, param_index as i32) } {
				0 => None,
				n => Some(Error::new(ErrorKind::Other, get_error(self.p_con.p_db, n).as_ref())) }
		}
		}
	}
}


impl<'a, 'b> Cursor<'a, 'b> {
	///Retrieve the column value as int with index <i>column_index</i>from the current row, the first column is 0.
	pub fn get_int(&self, column_index : i32) -> i32 {
		match self.p_stmt.p_con.db_type {
		DbType::SQLite3 => {
		unsafe { sqlite3_column_int(self.p_stmt.p_stmt, column_index as i32) as i32} 
		}
		}
	}
	///Retrieve the column value as i64 with index <i>column_index</i>from the current row, the first column is 0.
	pub fn get_long(&self, column_index : i32) -> i64 {
		match self.p_stmt.p_con.db_type {
		DbType::SQLite3 => {
		unsafe { sqlite3_column_int64(self.p_stmt.p_stmt, column_index as i32) } 
		}
		}
	}
	///Retrieve the column value as float with index <i>column_index</i>from the current row, the first column is 0.
	pub fn get_float(&self, column_index : i32) -> f32 {
		match self.p_stmt.p_con.db_type {
		DbType::SQLite3 => {
		unsafe { sqlite3_column_double(self.p_stmt.p_stmt, column_index as i32) as f32} 
		}
		}
	}
	///Retrieve the column value as double with index <i>column_index</i>from the current row, the first column is 0.
	pub fn get_double(&self, column_index : i32) -> f64 {
		match self.p_stmt.p_con.db_type {
		DbType::SQLite3 => {
		unsafe { sqlite3_column_double(self.p_stmt.p_stmt, column_index as i32) } 
		}
		}
	}

	///Retrieve the column value as String with index <i>column_index</i>from the current row, the first column is 0.
	pub fn get_string(&self, column_index : i32) -> String {
		match self.p_stmt.p_con.db_type {
		DbType::SQLite3 => {
			//match unsafe{CString::new(sqlite3_column_text(self.p_stmt.p_stmt, column_index as i32) as *const i8, false)}.as_str()
			match from_utf8(unsafe{CStr::from_ptr(sqlite3_column_text(self.p_stmt.p_stmt, column_index) as *const i8)}.to_bytes())
			{ Err(_) => String::new(), Ok(s) => s.to_string() }
		}
		}
	}

	///Retrieve the column value as an array of bytes <i>column_index</i>from the current row, the first column is 0.
	pub fn get_blob(&self, column_index : i32) -> Vec<u8> {
		match self.p_stmt.p_con.db_type {
		DbType::SQLite3 => {
		let p = unsafe { sqlite3_column_blob(self.p_stmt.p_stmt, column_index as i32) };
		let n = unsafe { sqlite3_column_bytes(self.p_stmt.p_stmt, column_index as i32) };
		Vec::from(unsafe {slice::from_raw_parts(p, n as usize)})
		}
		}
	}	
}

/// Allow to iterate Cursor.
impl<'a, 'b> Iterator for Cursor<'a, 'b> {
	type Item = Result<Cursor<'a, 'b>>;
	/// Returns the next row of the Cursor.
	///
	///Returns a Cursor if ok, or a <i>OtherIoError</i> IoError with (if available from the underlying database)
	///in the <i>detail</i> field text that describes the error, result code, and text that describes the result code.
	fn next(&mut self) -> Option<Result<Cursor<'a, 'b>>> {
		match self.p_stmt.p_con.db_type {
		DbType::SQLite3 => {
		if self.error { return None; }
		match unsafe { sqlite3_step(self.p_stmt.p_stmt) } {
			100 => Some(Ok(Cursor { p_stmt : self.p_stmt, error : false })),
			101 => None,
			err => {	self.error = true;
					Some (Err(Error::new(ErrorKind::Other, get_error(self.p_stmt.p_con.p_db, err).as_ref()))) } }
		}
		}
	}
}

impl Connection {
	///Open a new connection to the a database.
	///
	///Returns a Connection if ok, or a <i>ConnectionFailed</i> IoError with (if available from the underlying database)
	///in the <i>detail</i> field text that describes the error, result code, and text that describes the result code.
	pub fn new(db_type : DbType, filename : &str) -> Result<Connection> {
		match db_type {
			DbType::SQLite3 => {
				let p_db : *const i8 = null();
				match CString::new(filename) {
					Ok(s)	=> match unsafe{sqlite3_open(s.as_ptr(), &p_db)} {
								0 => Ok( Connection { 	db_type : DbType::SQLite3, p_db : p_db } ),
								i => Err(Error::new(ErrorKind::ConnectionRefused, get_error(p_db, i).as_ref())) },
					Err(e)	=> Err(Error::new(ErrorKind::Other, e))
				}
			}
		}
	}

	///Prepare a statement for executing SQL instructions.
	///
	///Returns a Statement if ok, or an <i>InvalidInput</i> IoError with (if available from the underlying database)
	///in the <i>detail</i> field text that describes the error, result code, and text that describes the result code.	
	pub fn prepare_statement<'a>(&'a self, sql :&str) -> Result<Statement<'a>> {
		match self.db_type {
			DbType::SQLite3 => {
				let p_stmt  : *const i8 = null();
				let pz_tail : *const i8 = null();
		
				match CString::new(sql) {
					Ok(s)	=> match unsafe { sqlite3_prepare_v2(self.p_db, s.as_ptr(), -1, &p_stmt, &pz_tail) } {
								0 => Ok(Statement { p_con : self, p_stmt : p_stmt, exec : false }),
								e => Err(Error::new(ErrorKind::InvalidInput, get_error(self.p_db, e).as_ref())) },
					Err(e)	=> Err(Error::new(ErrorKind::Other, e))
				}
			}
		}
	}

}

impl Drop for Connection {
	///The drop method is called when Connection goes out of scope, and therefore close properly the connection.
	fn drop(&mut self) {
		match self.db_type {
			DbType::SQLite3 => { if !self.p_db.is_null() { unsafe { sqlite3_close_v2(self.p_db); } } }
		}
	}
}

/*
//https://github.com/mozilla/rust/issues/13853
impl<'a> Drop for Statement<'a> {
	fn drop(&mut self) {}
}
*/

fn get_error<'a>(p_db : *const i8, errno : i32) -> String {
	let mut buf=String::new();	
	match from_utf8( unsafe{CStr::from_ptr(sqlite3_errmsg(p_db))}.to_bytes() ) {
		Ok(s) => buf.push_str(s),
		Err(_) => ()
	}
	if !buf.is_empty() { buf.push(' '); }
	buf.push('('); buf.push_str(errno.to_string().as_ref());
	match from_utf8( unsafe{CStr::from_ptr(sqlite3_errstr(errno))}.to_bytes() ) {
		Ok(s) => { buf.push(':');  buf.push_str(s); }
		Err(_) => ()
	}
	buf.push(')');
	buf
}