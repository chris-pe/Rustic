//!Miscellaneous utility classes.
use collections::hashmap::HashMap;
use std::io::{BufferedReader, IoResult};

/// Contains a list of properties. A property is a name and value pair.
pub struct Properties {
	props : HashMap<~str, ~str>
}

impl Properties {
	/// Create an empty properties list.
	pub fn new() -> Properties {
		Properties { props : HashMap::new() }
	}
	
	/// Add or update (if it already exists) a property.
	pub fn setProperty(&mut self, key : ~str, value : ~str) {
		self.props.insert(key, value);
	}
	
	/// Get a property value giving its name. Return None if property does not exist.
	pub fn getProperty<'a>(&'a self, key : &~str) -> Option<&'a ~str> {
		self.props.find(key)
	}
	
	/// Load properties from an UTF-8 input character stream (for example, but not restricted to, file).
	///
	/**Reader is already buffered during reading; so before invoking this method, there is no need for any additional BufferedReader to wrap around the reader.
	This method load properties from an input character stream processed in term of lines according to the following rules&nbsp;:
	<ul>
	<li>leading white-spaces (white-space Unicode definition) are skipped.</li>
	<li>line that contains only white-spaces is considered blank and is ignored</li>
	<li>if the the first non-white character is a '#' or '!', line is considered as a comment and is skipped.</li>
	<li>the key contains all of the characters in the line starting with the first non-white space character and up to,
	but not including, the first '=', ':' or white-space character other than a line terminator</li>
	<li>after the key, the first occurrence of '=' or ':' and all white-spaces are skipped</li>
	<li>the value contains all the remaining characters others than a line terminator</li>
	</ul>
	<u>Notes&nbsp;:</u>
	<ul>
	<li>a line terminator is either '\n' or '\r' or '\r\n'</li>
	<li>the following characters can be escaped&nbsp;: ASCII white-space '&#92;&nbsp;', line terminator '\r' or '\n', '&#92;#', '&#92;!', '\=', '&#92;:'</li>
	<li>a key-element pair may be spread out across several adjacent natural lines by terminating the line with a backslash character '\'</li>
	<pre class='rust fn'>targetCities=\
        Detroit, \
        Chicago, \
        Los Angeles</pre>
	is equivalent to <pre class='rust fn'>targetCities=Detroit, Chicago, Los Angeles</pre>
	</ul>
	*/
	pub fn load<T : Reader>(&mut self,  reader : T)-> IoResult<uint> {
		let mut t : IoResult<uint> = Ok(0); // Result<T, IoError>
		//let mut key;
		for line in BufferedReader::new(reader).lines() {
			match line {
				Ok(l) => {
					let mut k = l.trim_left(); if k.char_len()==0 { continue; }
					let mut idx : uint = 0;
					for c in k.chars() {if !c.is_whitespace() && c!='=' && c!=':' { idx=idx+1; continue; } }
					let key = k.slice_to(idx); k=k.slice_from(idx); k = k.trim_left();
					if k.char_len()==0 { self.props.insert(key.to_owned(), ~""); println!("'{}'=''", key); continue; }
					if k.char_at(0) == '=' || k.char_at(0) == '=' {
						k=k.slice_from(1);
						k=k.trim_left();
					}
					if k.char_at_reverse(0)=='\n' ||  k.char_at_reverse(0)=='\r' {
						if k.char_at_reverse(0)=='\r' { k=k.slice_to(k.char_len()-1); } // Line ends with '\r'
						else if k.char_len()>1 && k.char_at_reverse(1)=='\r' { k=k.slice_to(k.char_len()-2); } // Line ends with '\r\n'
							else { k=k.slice_to(k.char_len()-1); } // Line ends with '\n'
					}
					self.props.insert(key.to_owned(), k.to_owned());
					println!("'{}'='{}'", key, k);
				}
				Err(e) => { t=Err(e); break; }
			}
		}
		t
	}
}
