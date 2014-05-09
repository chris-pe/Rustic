//!Complementary crate to standard crates providing basic and easy to use functionalities.
//!
//!This project is at an early development stage, please do not use elements tagged as <i>WIP</i>. 
#![crate_id = "rustic#0.01"]
#![crate_type = "lib"]

#![desc = "Rustic"]
#![license = "GPLv2"]
#![comment = "Miscellaneous utility classes for the Rust programming language."]


extern crate collections;

///Miscellaneous utility classes.
pub mod util {
	pub use self::properties::Properties;
	mod properties;
}

///<b>WIP&nbsp;: </b>Provides the API for accessing and processing data stored in a data source (usually a relational database) using the Rust programming language.
pub mod sql {
	pub use self::connection::Connection;
	mod connection;
	///Supported Databases
	pub enum DbType {
		SQLITE
	}
}
