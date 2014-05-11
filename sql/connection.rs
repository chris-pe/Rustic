use libc::{c_int,c_char,c_uchar};
use std::ptr::RawPtr;
use std::c_str::CString;
use std::io::{IoResult,IoError,ConnectionFailed,InvalidInput};
use sql::{DbType};

#[link(name = "sqlite3")]
extern {
    fn sqlite3_open(filename : *c_char, ppDb : **mut()) -> c_int;
	fn sqlite3_close_v2(pDb : *mut()) -> c_int;
	fn sqlite3_errmsg(pDb : *mut()) -> *c_char;
	fn sqlite3_errstr(erno : c_int) -> *c_char;
	fn sqlite3_prepare_v2(pDb : *mut(), sql : *c_char, nByte : c_int, ppStmt : **mut(), pzTail : **()) -> c_int;
	fn sqlite3_finalize(pStmt : *mut()) -> c_int;
	fn sqlite3_step(pStmt : *mut()) -> c_int;
	fn sqlite3_column_text(pStmt : *mut(), iCol : c_int) -> *c_uchar;
	//fn sqlite3_column_int(pStmt : *mut(), iCol : c_int) -> c_int;
}

///Statement is used for executing SQL instructions and returning the results it produces.
pub struct Statement {
	dbType : DbType,
	pDb : *mut(),
	pStmt : *mut()
}

impl Statement {
	pub fn execute(&self) -> ResultSet {
		ResultSet { dbType : self.dbType, pDb : self.pDb, pStmt : self.pStmt }
	}
}

///ResultSet is used for representing a database query result.
pub struct ResultSet {
	dbType : DbType,
	pDb : *mut(),
	pStmt : *mut()	
}

impl ResultSet {
	pub fn get_string(&mut self, column_index : int) -> ~str {
		match self.dbType {
		SQLITE3 => {
		let retC;
		unsafe { retC = CString::new(sqlite3_column_text(self.pStmt, column_index as c_int) as *i8, false) };
		match retC.as_str() { None => return ~"", Some(s) => return s.into_owned() }
		}
		}
	}
}

/// Allow to iterate ResultSet.
impl Iterator<ResultSet> for ResultSet {
	fn next(&mut self) -> Option<ResultSet> {
		match self.dbType {
		SQLITE3 => {
		if unsafe { sqlite3_step(self.pStmt) } == 100 	{ return Some(*self) }
												else	{ return None }
		}
		}
	}
}

///Connection permits to connect to supported databases.
pub struct Connection {
	dbType : DbType,
	pDb : *mut()
}

impl Connection {
	///Open a new connection to the a database.
	///
	///Returns a Connection if ok, or a <i>ConnectionFailed</i> IoError with (if available from the underlying database)
	///in the <i>detail</i> field text that describes the error, result code, and text that describes the result code.
	pub fn new(dbType : DbType, filename : &str) -> IoResult<Connection> {
		match dbType {
		SQLITE3 => {
		let pDb : *mut () = RawPtr::null();
		let res = filename.with_c_str(|c_str| unsafe { sqlite3_open(c_str, &pDb) });
		if res==0	{ Ok( Connection { dbType : dbType, pDb : pDb } ) }
		else { 	let mut des = ~""; let mut det = ~""; 
				if pDb.is_not_null()	{ 	
					match (unsafe { CString::new(sqlite3_errmsg(pDb), false) }).as_str() { None => (), Some(s) => des=s.into_owned() }
				}
				match (unsafe { CString::new(sqlite3_errstr(res), false) }).as_str() { None => (), Some(s) => det=s.into_owned() }
				Err(IoError {	kind : ConnectionFailed,
								desc : "Database Connection Failed",
								detail : Some(des.append(" (").append(res.to_str()).append(":").append(det).append(")"))})
			}				
		}
		}
	}
	///Prepare a statement for executing SQL instructions.
	///
	///Returns a Statement if ok, or an <i>InvalidInput</i> IoError with (if available from the underlying database)
	///in the <i>detail</i> field text that describes the error, result code, and text that describes the result code.
	pub fn prepare_statement(&self, sql :&str) -> IoResult<Statement> {
		match self.dbType {
		SQLITE3 => {
		let pStmt  : *mut() = RawPtr::null();
		let pzTail : *() = RawPtr::null();
		let res = sql.with_c_str(|c_str| unsafe { sqlite3_prepare_v2(self.pDb, c_str, -1, &pStmt, &pzTail) });
		if res==0 { Ok(Statement { dbType : self.dbType, pDb : self.pDb, pStmt : pStmt }) }
		else { 	let mut des = ~""; let mut det = ~"";
				match (unsafe {	CString::new(sqlite3_errmsg(self.pDb), false) }).as_str() { None => (), Some(s) => des=s.into_owned() }
				match (unsafe { CString::new(sqlite3_errstr(res), false) }).as_str() { None => (), Some(s) => det=s.into_owned() }
				Err(IoError {	kind : InvalidInput, desc : "Statement Creation Failed",
								detail : Some(des.append(" (").append(res.to_str()).append(":").append(det).append(")"))}) }
		}
		}
	}
}

///The drop method is called when Connection goes out of scope, and therefore close properly the connection.
impl Drop for Connection {
	fn drop(&mut self) {
		match self.dbType {
			SQLITE3 => if self.pDb.is_not_null() { unsafe { sqlite3_close_v2(self.pDb); } }
		}
	}
}

impl Drop for Statement {
	fn drop (&mut self) {
		match self.dbType {
			SQLITE3 => if self.pStmt.is_not_null() { unsafe { sqlite3_finalize(self.pStmt); } }
		}
	}
}