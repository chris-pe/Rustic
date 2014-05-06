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
	<li>a line terminator is either '\n' or '\r\n'</li>
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
		let mut multi = ~"";
		for line in BufferedReader::new(reader).lines() {
			match line {
				Ok(l) => {
					multi = multi.append(l.trim_left());
					if multi.starts_with("#") || multi.starts_with("!") { multi=~""; continue; } // Comment line

					let multic = multi.clone();
					let mut buf = multic.slice_from(0); // How to convert ~str to &str ???
					if buf.ends_with("\n") {
						if buf.ends_with("\r\n") { buf=buf.slice_to(buf.len()-2); } // Line ends with '\r\n'
							else { buf=buf.slice_to(buf.len()-1); } // Line ends with '\n'
					}
					if buf.len()==0 { multi=~""; continue; } //Empty line
					
					// Special characters conversion
					buf.replace("\\\\")
					
					if buf.ends_with("\\") { multi = buf.slice_to(buf.len()-1).to_owned(); continue; }

					let mut idx : uint = 0u;
					for c in buf.chars() { if !c.is_whitespace() && c!='=' && c!=':' { idx+=1; }
											else { break; } }
					let key = buf.slice_chars(0, idx);
					buf = buf.slice_chars(idx, buf.char_len()); buf = buf.trim_left();
					if buf.starts_with("=") || buf.starts_with(":") { buf=buf.slice_from(1); buf = buf.trim_left(); }					
					self.props.insert(key.to_owned(), buf.to_owned()); println!("'{}'='{}'", key, buf);
					//println!("DEBUG:{}'", l);
				}
				Err(e) => { t=Err(e); break; }
			}
			//println!("DEBUG:{}'", multi);
			multi=~"";
		}
		if t.is_ok() { t = Ok(self.props.len()); }
		t
	}
}
