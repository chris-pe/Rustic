//!Miscellaneous utility classes.
use collections::hashmap::{HashMap, Keys, Entries, Values};
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
	<li>the following characters can be escaped&nbsp;: tab '\t', form feed '\f', line terminators '\r' or '\n'</li>
	<li>'\' before a a non-valid escape character is not an error, the backslash is simply dropped; useful to escape '\\\', '\ ', '\\#', '\\!', '\=', '\\:'</li>
	<li>a key-element pair may be spread out across several adjacent natural lines by terminating the line with a backslash character '\'</li>
	<pre class='rust fn'>targetCities=\
        Detroit, \
        Chicago, \
        Los Angeles</pre>
	is equivalent to <pre class='rust fn'>targetCities=Detroit, Chicago, Los Angeles</pre>
	</ul>
	*/
	pub fn load<T : Reader>(&mut self,  reader : T)-> IoResult<uint> {
		let mut ioresult : IoResult<uint> = Ok(0);
		let mut multi = ~"";
		for line in BufferedReader::new(reader).lines() {
			match line {
				Ok(l) => {
					multi = multi.append(l.trim_left());
					if multi.starts_with("#") || multi.starts_with("!") { multi=~""; continue; } // Comment line

					if multi.ends_with("\n") {
						if multi.ends_with("\r\n") { multi=multi.slice_to(multi.len()-2).to_owned(); } // Line ends with '\r\n'
							else { multi=multi.slice_to(multi.len()-1).to_owned(); } // Line ends with '\n'
					}
					if multi.len()==0 { multi=~""; continue; } //Empty line
					
					// line finishing with and off number of '\' is a multiline
					let mut esc = false;
					for c in multi.clone().chars_rev() {
						if c=='\\' { esc=!esc; } else { break; }
					}
					if esc { multi = multi.slice_to(multi.len()-1).to_owned(); continue; }

					let mut idx = 0u;
					for c in multi.chars() { if !c.is_whitespace() && c!='=' && c!=':' { idx+=c.len_utf8_bytes(); }
											else { break; } }
					let key = multi.slice_to(idx).to_owned();
					multi = multi.slice_from(idx).to_owned(); multi = multi.trim_left().to_owned();
					if multi.starts_with("=") || multi.starts_with(":") { multi=multi.slice_from(1).to_owned(); multi = multi.trim_left().to_owned(); }					
					self.props.insert(escapeChars(key), escapeChars(multi)); //println!("'{}'='{}'", key,multi);
					
					// println!("DEBUG:'{}'", multi); 

				}
				Err(e) => { ioresult=Err(e); break; }
			}
			//println!("DEBUG:{}'", multi);
			multi=~"";
		}
		if ioresult.is_ok() { ioresult = Ok(self.props.len()); }
		ioresult
	}

	/*pub fn store<T : Writer>(&mut self,  _writer : T) {
		for i in self.props.iter() {
			println!("{}", i);
		}
	}*/

	/// An iterator visiting all properties keys in arbitrary order. Iterator element type is &'a ~str.
	pub fn keys<'a>(&'a self) -> Keys<'a, ~str, ~str> {
		self.props.keys()
	}
	
	/// An iterator visiting all properties values in arbitrary order. Iterator element type is &'a ~str.
	pub fn values<'a>(&'a self) -> Values<'a, ~str, ~str> {
		self.props.values()
	}

	/// An iterator visiting all properties key-value pairs in arbitrary order. Iterator element type is (&'a ~str, &'a ~str).
	pub fn iter<'a>(&'a self) -> Entries<'a, ~str, ~str> {
		self.props.iter()
	}
	
	/// Clear the properties, removing all key-value pairs.
	pub fn clear(&mut self) {
		self.props.clear();
	}

}

fn escapeChars(mut s : ~str) -> ~str {
	// Escape \t \f \r \n
	let mut esc=false;
	let mut idx : uint = 0u;
	for mut c in s.clone().chars() {
		if esc {
			match c {
				't' => { s=s.slice_to(idx).to_owned().append("\t").append(s.slice_from(idx+2)); esc=false }
				'f' => { s=s.slice_to(idx).to_owned().append("\x0c").append(s.slice_from(idx+2)); esc=false }
				'r' => { s=s.slice_to(idx).to_owned().append("\r").append(s.slice_from(idx+2)); esc=false }
				'n' => { s=s.slice_to(idx).to_owned().append("\n").append(s.slice_from(idx+2)); esc=false }
				_  => { s=s.slice_to(idx).to_owned().append(s.slice_from(idx+1)); c=' '; esc=false }
			}
		}
		if c=='\\' { esc=true; continue; }
		idx+=c.len_utf8_bytes();
	}
	s
	}