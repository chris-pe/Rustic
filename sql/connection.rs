use libc::{c_int,c_char,c_uchar};
use std::ptr::RawPtr;
use std::c_str::CString;
use std::io::{IoResult,IoError,ConnectionFailed,InvalidInput,OtherIoError};
use sql::{DbType};

#[link(name = "sqlite3")]
extern {
    fn sqlite3_open(filename : *c_char, ppDb : **mut()) -> c_int;
	fn sqlite3_close_v2(pDb : *mut()) -> c_int;
	fn sqlite3_errmsg(pDb : *mut()) -> *c_char;
	fn sqlite3_errstr(erno : c_int) -> *c_char;
	fn sqlite3_prepare_v2(pDb : *mut(), sql : *c_char, nByte : c_int, ppStmt : **mut(), pzTail : **()) -> c_int;
	//fn sqlite3_finalize(pStmt : *mut()) -> c_int;
	fn sqlite3_step(pStmt : *mut()) -> c_int;
	fn sqlite3_column_text(pStmt : *mut(), iCol : c_int) -> *c_uchar;
	//fn sqlite3_column_int(pStmt : *mut(), iCol : c_int) -> c_int;
}

///Connection permits to connect to supported databases.
pub struct Connection {
	dbType : DbType,
	pDb : *mut()
}

///Statement is used for executing SQL instructions and returning the results it produces.
pub struct Statement<'a> {
	pCon  : &'a Connection,
	pStmt : *mut()
}

///ResultSet is used for representing a database query result.
pub struct ResultSet<'a> {
	pStmt : &'a Statement<'a>,
	error : bool
}

impl<'a> Statement<'a> {
	///Execute the SQL query and returns the result in an iterable ResultSet.
	pub fn execute_query(&'a self) -> ResultSet<'a> {
		ResultSet { pStmt : self, error : false }
	}
	pub fn execute(&'a self) -> Option<IoError> {
		match self.pCon.dbType {
		SQLITE3 => {
		let res = unsafe { sqlite3_step(self.pStmt) };
		if res != 100 && res != 101	{
			Some (IoError {	kind : OtherIoError, desc : "Row Fetch Failed",
							detail : Some(get_error(self.pCon.pDb, res))}) }
		else { return None; }
		}
		}
	}
}

impl<'a> ResultSet<'a> {
	///Retrieve the column value with index <i>column_index</i>from the current row, the first column is 0.
	pub fn get_string(&mut self, column_index : int) -> ~str {
		match self.pStmt.pCon.dbType {
		SQLITE3 => {
		let retC;
		retC = unsafe { CString::new(sqlite3_column_text(self.pStmt.pStmt, column_index as c_int) as *i8, false) };
		if retC.is_null() { return ~""; };
		match retC.as_str() { None => return ~"", Some(s) => return s.into_owned() }
		}
		}
	}
}

/// Allow to iterate ResultSet.
impl<'a> Iterator<IoResult<ResultSet<'a>>> for ResultSet<'a> {
	/// Returns the next row of the ResultSet.
	///
	///Returns a ResultSet if ok, or a <i>OtherIoError</i> IoError with (if available from the underlying database)
	///in the <i>detail</i> field text that describes the error, result code, and text that describes the result code.
	fn next(&mut self) -> Option<IoResult<ResultSet<'a>>> {
		match self.pStmt.pCon.dbType {
		SQLITE3 => {
		if self.error { return None }
		let res = unsafe { sqlite3_step(self.pStmt.pStmt) };
		if res == 100 	{ Some(Ok(*self)) } else
		if res == 101	{ None } else
		{	self.error = true;
			Some (Err(IoError {	kind : OtherIoError, desc : "Row Fetch Failed",
								detail : Some(get_error(self.pStmt.pCon.pDb, res))})) }
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
		SQLITE3 => {
		let pDb : *mut () = RawPtr::null();
		let res = filename.with_c_str(|c_str| unsafe { sqlite3_open(c_str, &pDb) });
		if res==0	{ 	Ok( Connection { dbType : dbType, pDb : pDb } ) }
		else 		{ 	Err(IoError {	kind : ConnectionFailed, desc : "Database Connection Failed",
								detail : Some(get_error(pDb, res))}) }				
		}
		}
	}
	///Prepare a statement for executing SQL instructions.
	///
	///Returns a Statement if ok, or an <i>InvalidInput</i> IoError with (if available from the underlying database)
	///in the <i>detail</i> field text that describes the error, result code, and text that describes the result code.
	pub fn prepare_statement<'a>(&'a self, sql :&str) -> IoResult<Statement<'a>> {
		match self.dbType {
		SQLITE3 => {
		let pStmt  : *mut() = RawPtr::null();
		let pzTail : *() = RawPtr::null();
		let res = sql.with_c_str(|c_str| unsafe { sqlite3_prepare_v2(self.pDb, c_str, -1, &pStmt, &pzTail) });
		if res==0	{	Ok(Statement { pCon : self, pStmt : pStmt }) }
			else 	{	Err(IoError {	kind : InvalidInput, desc : "Statement Creation Failed",
										detail : Some(get_error(self.pDb, res))}) }
		}
		}
	}
}

impl Drop for Connection {
	///The drop method is called when Connection goes out of scope, and therefore close properly the connection.
	fn drop(&mut self) {
		match self.dbType {
			SQLITE3 => if self.pDb.is_not_null() { unsafe { sqlite3_close_v2(self.pDb); } }
		}
	}
}

/*impl<'a> Drop for Statement<'a> {
	///The drop method is called when Statement goes out of scope, and therefore delete properly the Statement.
	fn drop (&mut self) {
		match self.pCon.dbType {
			SQLITE3 => if self.pStmt.is_not_null() { unsafe { sqlite3_finalize(self.pStmt); } }
		}
	}
}*/

fn get_error(pDb : *mut(), errno : c_int) -> ~str {
	let mut des = ~""; let mut det = ~"";
	let desC = unsafe {	CString::new(sqlite3_errmsg(pDb), false) };
	if desC.is_not_null() { match desC.as_str() { None => (), Some(s) => des=s.into_owned() } }
	let detC = unsafe { CString::new(sqlite3_errstr(errno), false) }; 
	if detC.is_not_null() { match detC.as_str() { None => (), Some(s) => det=s.into_owned() } }
	des.append(" (").append(errno.to_str()).append(":").append(det).append(")")
}