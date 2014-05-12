use collections::hashmap::{HashMap, Keys, Entries, Values, MutEntries};
use std::io::{BufferedReader, BufferedWriter, IoError};

///Contains a list of properties. A property is a key-value pair.
pub struct Properties {
	props : HashMap<~str, ~str>
}

impl Map<~str, ~str> for Properties {
	///Get a property value giving its name. Return None if property does not exist.
	fn find<'a>(&'a self, key : &~str) -> Option<&'a ~str> {
		self.props.find(key)
	}	
	///Return true if a property value exists for the specified key
	fn contains_key(&self, key: &~str) -> bool {
		self.props.contains_key(key)
	}
}

impl Container for Properties {
	///Return the number of properties.
	fn len(&self) -> uint {
		self.props.len()
	}	
	///Return true if the properties list is empty
	fn is_empty(&self) -> bool {
		self.props.is_empty()
	}
}

impl Mutable for Properties {
	///Remove all properties.
	fn clear(&mut self) {
		self.props.clear();
	}
}

impl MutableMap<~str, ~str> for Properties {
	///Insert a property into the list. If the property already had a value present in the list, that value is returned.
	///Otherwise None is returned.
	fn swap(&mut self, key: ~str, value: ~str) -> Option<~str> {
		self.props.swap(key, value)
	}

	///Removes a property from the list, returning the value of the property if it was previously in the list.
	fn pop(&mut self, key: &~str) -> Option<~str> {
		self.props.pop(key)
	}

	///Return a mutable reference to the value corresponding to the property
	fn find_mut<'a>(&'a mut self, key: &~str) -> Option<&'a mut ~str> {
		self.props.find_mut(key)
	}

	///Insert a property into the list. An existing value for a property is replaced by the new value.
	///Return true if the property did not already exist in the list.
	fn insert(&mut self, key: ~str, value: ~str) -> bool {
		self.props.insert(key, value)
	}

	/// Remove a property from the list. Return true if the property was present in the list, otherwise false.
	fn remove(&mut self, key: &~str) -> bool {
		self.props.remove(key)
	}
}

impl Properties {
	/// Create an empty properties list.
	pub fn new() -> Properties {
		Properties { props : HashMap::new() }
	}
	
	/// Load properties from an UTF-8 input character stream (for example, but not restricted to, file).
	///
	/**Reader is already buffered during reading; so before invoking this method,
	there is no need for any additional BufferedReader to wrap around the reader.
	This method load properties from an input character stream processed in term of lines according to the following rules&nbsp;:
	<ul>
	<li>leading white-spaces (Unicode definition) are skipped.</li>
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
	<li>'\' before a non-valid escape character is not an error, the backslash is simply dropped;
	useful to escape '\\\', '\ ', '\\#', '\\!', '\=', '\\:'</li>
	<li>a key-element pair may be spread out across several adjacent lines by terminating the line with a backslash character '\'</li>
	<pre class='rust fn'>targetCities=\
        Detroit, \
        Chicago, \
        Los Angeles</pre>
	is equivalent to <pre class='rust fn'>targetCities=Detroit, Chicago, Los Angeles</pre>
	</ul>
	*/
	pub fn load<T : Reader>(&mut self,  reader : T)-> Option<IoError> {
		let mut multi = ~"";
		for line in BufferedReader::new(reader).lines() {
			match line {
				Ok(l) => {
					multi = multi.append(l.trim_left());
					if multi.starts_with("#") || multi.starts_with("!") { multi=~""; continue; } // Comment line

					if multi.ends_with("\n") {
						if multi.ends_with("\r\n") { multi=multi.slice_to(multi.len()-2).into_owned(); } // Line ends with '\r\n'
							else { multi=multi.slice_to(multi.len()-1).into_owned(); } // Line ends with '\n'
					}
					if multi.len()==0 { multi=~""; continue; } //Empty line
					
					// line finishing with an odd number of '\' is a multiline
					let mut esc = false;
					for c in multi.chars_rev() {
						if c=='\\' { esc=!esc; } else { break; }
					}
					if esc { multi = multi.slice_to(multi.len()-1).to_owned(); continue; }

					// determination of the key
					esc=false;
					let mut idx = 0u;							
					for c in multi.chars() {
						if c=='\\' { esc=true; idx+=1; continue; }
						if !esc && (c.is_whitespace() || c=='=' || c==':') { break; } 
						esc=false; idx+=c.len_utf8_bytes();
					}
					let key = multi.slice_to(idx).to_owned();
					
					multi = multi.slice_from(idx).to_owned(); multi = multi.trim_left().to_owned();
					if multi.starts_with("=") || multi.starts_with(":") { 	multi=multi.slice_from(1).into_owned();
																			multi = multi.trim_left().into_owned(); }					
					self.props.insert(decode_chars(key), decode_chars(multi));
				}
				Err(e) => { return Some(e); }
			}
			multi=~"";
		}
		None
	}

	///Store properties to an UTF-8 output character stream (for example, but not restricted to, file) suitable for loading
	///into a Properties list using the <code><b>fn <a href="#method.load" class="fnname">load</a>&lt;T:
	///<a class="trait" href="http://static.rust-lang.org/doc/master/std/io/trait.Reader.html"
	///title="std::io::Reader">Reader</a>&gt;</b></code> method.
	pub fn store<T : Writer>(&mut self,  writer : T) -> Option<IoError> {
		let mut buf = BufferedWriter::new(writer);
		for kv in self.props.iter() {
			match kv {
				(k,v) => match buf.write_line(encode_chars(k.to_owned(), true)
						.append("=").append(encode_chars(v.to_owned(), false))) {
							Ok(_)  => continue,
							Err(e) => { return Some(e); }
				}
			}
		}
		None
	}

	///An iterator visiting all properties keys in arbitrary order. Iterator element type is &'a ~str.
	pub fn keys<'a>(&'a self) -> Keys<'a, ~str, ~str> {
		self.props.keys()
	}
	
	///An iterator visiting all properties values in arbitrary order. Iterator element type is &'a ~str.
	pub fn values<'a>(&'a self) -> Values<'a, ~str, ~str> {
		self.props.values()
	}

	///An iterator visiting all properties key-value pairs in arbitrary order. Iterator element type is (&'a ~str, &'a ~str).
	pub fn iter<'a>(&'a self) -> Entries<'a, ~str, ~str> {
		self.props.iter()
	}
	
	///An iterator visiting all properties key-value pairs in arbitrary order, with mutable references to the values.
	///Iterator element type is (&'a ~str, &'a mut ~str).
	pub fn mut_iter<'a>(&'a mut self) -> MutEntries<'a, ~str, ~str> {
		self.props.mut_iter()
	}
}

fn decode_chars(mut s : ~str) -> ~str {
	// Escape \t \f \r \n
	let mut esc=false;
	let mut idx = 0u;
	for mut c in s.clone().chars() {
		if esc {
			match c {
				't' => { s=s.slice_to(idx).to_owned().append("\t").append(s.slice_from(idx+2)); esc=false; }
				'f' => { s=s.slice_to(idx).to_owned().append("\x0c").append(s.slice_from(idx+2)); esc=false; }
				'r' => { s=s.slice_to(idx).to_owned().append("\r").append(s.slice_from(idx+2)); esc=false; }
				'n' => { s=s.slice_to(idx).to_owned().append("\n").append(s.slice_from(idx+2)); esc=false; }
				'\\'=> { s=s.slice_to(idx).to_owned().append(s.slice_from(idx+1)); c=' '; esc=false; }
				 _  => { s=s.slice_to(idx).to_owned().append(s.slice_from(idx+1)); esc=false; }
			}
		}
		if c=='\\' { esc=true; continue; }
		idx+=c.len_utf8_bytes();
	}
	s
}

fn encode_chars(mut s : ~str, isKey : bool) -> ~str {
	let mut esc=true;
	let mut idx = 0u;
	for c in s.clone().chars() {
		if c.is_whitespace() 	{
			if esc 	{ 	match c {
							'\t'   => { s=s.slice_to(idx).to_owned().append("\\t").append(s.slice_from(idx+1)); idx+=1; }
							'\x0c' => { s=s.slice_to(idx).to_owned().append("\\f").append(s.slice_from(idx+1)); idx+=1; }
							'\r'   => { s=s.slice_to(idx).to_owned().append("\\r").append(s.slice_from(idx+1)); idx+=1; }
							'\n'   => { s=s.slice_to(idx).to_owned().append("\\n").append(s.slice_from(idx+1)); idx+=1; }
							_      => { s=s.slice_to(idx).to_owned().append("\\").append(s.slice_from(idx)); idx+=1; }
						}
			} else 	{ 	match c {
							'\r' => { s=s.slice_to(idx).to_owned().append("\\r").append(s.slice_from(idx+1)); idx+=1; }
							'\n' => { s=s.slice_to(idx).to_owned().append("\\n").append(s.slice_from(idx+1)); idx+=1; }
							_    => ()
						}
					}
		}
		else 	{	if c=='\\' { s=s.slice_to(idx).to_owned().append("\\").append(s.slice_from(idx)); idx+=1; }
					if !isKey { esc=false; }
					else if c=='=' || c==':' { s=s.slice_to(idx).to_owned().append("\\").append(s.slice_from(idx)); idx+=1; }
				}
		idx+=c.len_utf8_bytes();
	}
	if isKey && (s.starts_with("#") || s.starts_with("!")) {
		s=(~"\\").append(s);
	}
	s
}