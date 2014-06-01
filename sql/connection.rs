use libc::{c_int, c_char, c_uchar, c_double,c_void};
use std::c_str::CString;
use std::c_vec::CVec;
use std::vec::Vec;
use std::io::{IoResult, IoError, ConnectionFailed, InvalidInput, OtherIoError};
use std::ptr::null;
use sql::{DbType, SQLite3};

#[link(name = "sqlite3")]
extern {
    fn sqlite3_open(filename : *c_char, ppDb : **c_void) -> c_int;
	fn sqlite3_close_v2(pDb : *c_void) -> c_int;
	fn sqlite3_errmsg(pDb : * c_void) -> *c_char;
	fn sqlite3_errstr(erno : c_int) -> *c_char;
	fn sqlite3_prepare_v2(pDb : *c_void, sql : *c_char, nByte : c_int, ppStmt : **c_void, pzTail : **c_void) -> c_int;
	fn sqlite3_step(pStmt : *c_void) -> c_int;
	fn sqlite3_changes(pStmt : *c_void) -> c_int;
	fn sqlite3_column_int(pStmt : *c_void, iCol : c_int) -> c_int;
	fn sqlite3_column_int64(pStmt : *c_void, iCol : c_int) -> i64;
	fn sqlite3_column_double(pStmt : *c_void, iCol : c_int) -> c_double;
	fn sqlite3_column_text(pStmt : *c_void, iCol : c_int) -> *c_uchar;
	fn sqlite3_column_blob(pStmt : *c_void, iCol : c_int) ->  *mut u8;
	fn sqlite3_column_bytes(pStmt : *c_void, iCol : c_int) -> c_int;
	fn sqlite3_bind_int(pStmt : *c_void, iCol : c_int, value : c_int) -> c_int;
	fn sqlite3_bind_int64(pStmt : *c_void, iCol : c_int, value : i64) -> c_int;
	fn sqlite3_bind_double(pStmt : *c_void, iCol : c_int, value : f64) -> c_int;
	fn sqlite3_bind_text(pStmt : *c_void, iCol : c_int, value : *c_char, n : c_int, f: *extern fn(*c_void)) -> c_int;
	fn sqlite3_bind_null(pStmt : *c_void, iCol : c_int) -> c_int;
	fn sqlite3_bind_blob(pStmt : *c_void, iCol : c_int, value : *c_char, n : c_int, f: *extern fn(*c_void)) -> c_int;
	fn sqlite3_reset(pStmt : *c_void) -> c_int;
	//fn sqlite3_finalize(pStmt : *c_void) -> c_int;
}

///Connection permits to connect to supported databases.
pub struct Connection {
	dbType : DbType,
	pDb : *c_void
}

///Statement is used for executing SQL instructions and returning results.
///
///In the SQL Statement, ? character is replaced by a parameter using a set_* method.
pub struct Statement<'a> {
	pCon  : &'a Connection,
	pStmt : *c_void,
	exec  : bool
}

///Cursor is used for browsing a database query result.
pub struct Cursor<'a, 'b> {
	pStmt : &'b Statement<'a>,
	error : bool
}

impl<'a> Statement<'a> {
	///Execute the SQL query and returns the result in an iterable Cursor.
	pub fn execute_query<'b>(&'b mut self) -> Cursor<'a, 'b> {
		match self.pCon.dbType {
		SQLite3 => {
		if self.exec { unsafe { sqlite3_reset(self.pStmt) }; } else { self.exec=true; }
		Cursor { pStmt : self, error : false }
		}
		}
	}
	///Execute the SQL statement and returns None if succeeds or an IoError.
	pub fn execute(&mut self) -> Option<IoError> {
		match self.pCon.dbType {
		SQLite3 => {
		if self.exec { unsafe { sqlite3_reset(self.pStmt) }; } else { self.exec=true; }
		match unsafe { sqlite3_step(self.pStmt) } {
			100 | 101 => None,
			err => Some (IoError {	kind : OtherIoError, desc : "Statement Execution Failed",
								detail : Some(get_error(self.pCon.pDb, err))}) }
		}
		}
	}
	///Execute the SQL INSERT, UPDATE or DELETE statement and returns the number of affected rows.
	///Returns 0 for SQL statement that returns nothing. Returns an IoError if fails.
	pub fn execute_update(&mut self) -> IoResult<int> {
		match self.pCon.dbType {
		SQLite3 => { 
		if self.exec { unsafe { sqlite3_reset(self.pStmt) }; } else { self.exec=true; }
		match unsafe { sqlite3_step(self.pStmt) } {
			100 | 101 => Ok(unsafe { sqlite3_changes(self.pStmt) } as int),
			err => Err(IoError {	kind : OtherIoError, desc : "Statement Execution Failed",
								detail : Some(get_error(self.pCon.pDb, err)) }) }
		}
		}
	}
	///Replace in the SQL Statement the '?' parameter by an int. The leftmost parameter has an index of 1.
	pub fn set_int(&mut self, param_index : int, value : int) -> Option<IoError> {
		match self.pCon.dbType {
		SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.pStmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_int(self.pStmt, param_index as c_int, value as c_int) } {
				0 => None,
				n => Some (	IoError {	kind : OtherIoError, desc : "Statement Set Parameter Failed",
										detail : Some(get_error(self.pCon.pDb, n))}) }
			}
		}
	}
	///Replace in the SQL Statement the '?' parameter by an i64. The leftmost parameter has an index of 1.
	pub fn set_long(&mut self, param_index : int, value : i64) -> Option<IoError> {
		match self.pCon.dbType {
		SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.pStmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_int64(self.pStmt, param_index as c_int, value) } {
				0 => None,
				n => Some (	IoError {	kind : OtherIoError, desc : "Statement Set Parameter Failed",
										detail : Some(get_error(self.pCon.pDb, n))}) }
		}
		}
	}

	///Replace in the SQL Statement the '?' parameter by an f32. The leftmost parameter has an index of 1.
	pub fn set_float(&mut self, param_index : int, value : f32) -> Option<IoError> {
		match self.pCon.dbType {
		SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.pStmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_double(self.pStmt, param_index as c_int, value as f64) } {
				0 => None,
				n => Some (	IoError {	kind : OtherIoError, desc : "Statement Set Parameter Failed",
										detail : Some(get_error(self.pCon.pDb, n))}) }
		}
		}
	}

	///Replace in the SQL Statement the '?' parameter by a double. The leftmost parameter has an index of 1.
	pub fn set_double(&mut self, param_index : int, value : f64) -> Option<IoError> {
		match self.pCon.dbType {
		SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.pStmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_double(self.pStmt, param_index as c_int, value) } {
				0 => None,
				n => Some (	IoError {	kind : OtherIoError, desc : "Statement Set Parameter Failed",
										detail : Some(get_error(self.pCon.pDb, n))}) }
		}
		}
	}

	///Replace in the SQL Statement the '?' parameter by an &str. The leftmost parameter has an index of 1.
	pub fn set_string(&mut self, param_index : int, value : &str) -> Option<IoError> {
		match self.pCon.dbType {
		SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.pStmt) }; self.exec=false; }
			match value.with_c_str(|c_str| unsafe { sqlite3_bind_text(self.pStmt, param_index as c_int,
													c_str, -1, -1 as *extern fn(*c_void)) }) {
				0 => None,
				n => Some (	IoError {	kind : OtherIoError, desc : "Statement Set Parameter Failed",
										detail : Some(get_error(self.pCon.pDb, n as c_int))}) 
			}
		}
		}
	}
	///Replace in the SQL Statement the '?' parameter by an &[u8]. The leftmost parameter has an index of 1.
	pub fn set_blob(&mut self, param_index : int, value : Vec<u8>) -> Option<IoError> {
		match self.pCon.dbType {
		SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.pStmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_blob(self.pStmt, param_index as c_int, value.as_ptr() as *i8,
												value.len() as i32, -1 as *extern fn(*c_void)) } {
				0 => None,
				n => Some (	IoError {	kind : OtherIoError, desc : "Statement Set Parameter Failed",
										detail : Some(get_error(self.pCon.pDb, n))}) }
		}
		}
	}
	///Replace in the SQL Statement the '?' parameter by an SQL NULL. The leftmost parameter has an index of 1.
	pub fn set_null(&mut self, param_index : int) -> Option<IoError> {
		match self.pCon.dbType {
		SQLite3 => {
			if self.exec { unsafe { sqlite3_reset(self.pStmt) }; self.exec=false; }
			match unsafe { sqlite3_bind_null(self.pStmt, param_index as c_int) } {
				0 => None,
				n => Some (	IoError {	kind : OtherIoError, desc : "Statement Set Parameter Failed",
										detail : Some(get_error(self.pCon.pDb, n))}) }
		}
		}
	}
}

impl<'a, 'b> Cursor<'a, 'b> {
	///Retrieve the column value as int with index <i>column_index</i>from the current row, the first column is 0.
	pub fn get_int(&self, column_index : int) -> int {
		match self.pStmt.pCon.dbType {
		SQLite3 => {
		unsafe { sqlite3_column_int(self.pStmt.pStmt, column_index as c_int) as int} 
		}
		}
	}
	///Retrieve the column value as i64 with index <i>column_index</i>from the current row, the first column is 0.
	pub fn get_long(&self, column_index : int) -> i64 {
		match self.pStmt.pCon.dbType {
		SQLite3 => {
		unsafe { sqlite3_column_int64(self.pStmt.pStmt, column_index as c_int) } 
		}
		}
	}
	///Retrieve the column value as float with index <i>column_index</i>from the current row, the first column is 0.
	pub fn get_float(&self, column_index : int) -> f32 {
		match self.pStmt.pCon.dbType {
		SQLite3 => {
		unsafe { sqlite3_column_double(self.pStmt.pStmt, column_index as c_int) as f32} 
		}
		}
	}
	///Retrieve the column value as double with index <i>column_index</i>from the current row, the first column is 0.
	pub fn get_double(&self, column_index : int) -> f64 {
		match self.pStmt.pCon.dbType {
		SQLite3 => {
		unsafe { sqlite3_column_double(self.pStmt.pStmt, column_index as c_int) } 
		}
		}
	}
	///Retrieve the column value as String with index <i>column_index</i>from the current row, the first column is 0.
	pub fn get_string(&self, column_index : int) -> String {
		match self.pStmt.pCon.dbType {
		SQLite3 => {
		 	let c_str = unsafe { CString::new(sqlite3_column_text(self.pStmt.pStmt, column_index as c_int) as *i8, false) };
			if c_str.is_null() { return "".to_string(); };
			match c_str.as_str() { None => "".to_string(), Some(s) => s.to_string() }
		}
		}
	}

	///Retrieve the column value as an array of bytes <i>column_index</i>from the current row, the first column is 0.
	pub fn get_blob(&self, column_index : int) -> Vec<u8> {
		match self.pStmt.pCon.dbType {
		SQLite3 => {
		let p = unsafe { sqlite3_column_blob(self.pStmt.pStmt, column_index as c_int) };
		let n = unsafe { sqlite3_column_bytes(self.pStmt.pStmt, column_index as c_int) };
		let c_vec = unsafe { CVec::new(p, n as uint) };
		Vec::from_slice(c_vec.as_slice()).clone()
		}
		}
	}
}

/// Allow to iterate Cursor.
impl<'a, 'b> Iterator<IoResult<Cursor<'a, 'b>>> for Cursor<'a, 'b> {
	/// Returns the next row of the Cursor.
	///
	///Returns a Cursor if ok, or a <i>OtherIoError</i> IoError with (if available from the underlying database)
	///in the <i>detail</i> field text that describes the error, result code, and text that describes the result code.
	fn next(&mut self) -> Option<IoResult<Cursor<'a, 'b>>> {
		match self.pStmt.pCon.dbType {
		SQLite3 => {
		if self.error { return None; }
		match unsafe { sqlite3_step(self.pStmt.pStmt) } {
			100 => Some(Ok(*self)),
			101 => None,
			err => {	self.error = true;
					Some (Err(IoError {	kind : OtherIoError, desc : "Row Fetch Failed",
										detail : Some(get_error(self.pStmt.pCon.pDb, err))})) } }
		}
		}
	}
}

impl Connection {
	///Open a new connection to the a database.
	///
	///Returns a Connection if ok, or a <i>ConnectionFailed</i> IoError with (if available from the underlying database)
	///in the <i>detail</i> field text that describes the error, result code, and text that describes the result code.
	pub fn new(dbType : DbType, filename : &str) -> IoResult<Connection> {
		match dbType {
			SQLite3 => {
				let pDb : *c_void = null();
				match filename.with_c_str(|c_str| unsafe { sqlite3_open(c_str, &pDb) }) {
					0 => Ok( Connection { 	dbType : SQLite3, pDb : pDb } ),
					e => Err(IoError	{ 	kind : 	ConnectionFailed, desc : "Database Connection Failed",
											detail : Some(get_error(pDb, e))}) }
			}
		}
	}
	///Prepare a statement for executing SQL instructions.
	///
	///Returns a Statement if ok, or an <i>InvalidInput</i> IoError with (if available from the underlying database)
	///in the <i>detail</i> field text that describes the error, result code, and text that describes the result code.
	pub fn prepare_statement<'a>(&'a self, sql :&str) -> IoResult<Statement<'a>> {
		match self.dbType {
		SQLite3 => {
		let pStmt  : *c_void = null();
		let pzTail : *c_void = null();
		match sql.with_c_str(|c_str| unsafe { sqlite3_prepare_v2(self.pDb, c_str, -1, &pStmt, &pzTail) }) {
			0 => Ok(Statement { pCon : self, pStmt : pStmt, exec : false }),
			e => Err(IoError{	kind : InvalidInput, desc : "Statement Creation Failed",
								detail : Some(get_error(self.pDb, e))}) }
		}
		}
	}
}

impl Drop for Connection {
	///The drop method is called when Connection goes out of scope, and therefore close properly the connection.
	fn drop(&mut self) {
		match self.dbType {
			SQLite3 => { if self.pDb.is_not_null() { unsafe { sqlite3_close_v2(self.pDb); } } }
		}
	}
}

fn get_error(pDb : *c_void, errno : c_int) -> String {
	let mut buf = String::new();
	unsafe	{	let c_str = CString::new(sqlite3_errmsg(pDb), false);
				if c_str.is_not_null() { match c_str.as_str() { None => (), Some(s) => buf=buf.append(s).append(" ") } } }
	buf=buf.append("(").append(errno.to_str().as_slice());
	unsafe	{	let c_str = CString::new(sqlite3_errstr(errno), false);
				if c_str.is_not_null() { match c_str.as_str() { None => (), Some(s) => buf=buf.append(":").append(s) } } }
	buf.append(")")
}