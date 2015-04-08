//!Complementary crate to standard crates providing basic and easy to use functionalities.
//!
//!This project is at an early development stage, please do not use elements tagged as <i>WIP</i>. 
#![crate_type = "lib"]
#![crate_name = "rustic"]
#![feature(collections,libc,convert)]

extern crate libc;

///Miscellaneous utility classes.
pub mod util {
	pub use self::properties::Properties;
	mod properties;
}

///Provides the API for accessing and processing data stored in a relational database.
pub mod sql {
	pub use self::connection::Connection;
	pub use self::connection::Statement;
	pub use self::connection::Cursor;
	mod connection;
	///Supported Databases
	pub enum DbType {
		SQLite3
	}
	//impl Copy for DbType {}
}
