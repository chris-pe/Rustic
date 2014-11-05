use std::collections::hash_map::{HashMap, Keys, Entries, Values, MutEntries};
use std::io::{BufferedReader, BufferedWriter, IoError};

///Contains a list of properties. A property is a key-value pair.
pub struct Properties {
	props : HashMap<String, String>
}

impl Properties {
	/// Create an empty properties list.
	pub fn new() -> Properties {
		Properties { props : HashMap::new() }
	}
	
	///Return the number of properties.
	pub fn len(&self) -> uint {
		self.props.len()
	}	
	///Return true if the properties list is empty
	pub fn is_empty(&self) -> bool {
		self.props.is_empty()
	}

	///Remove all properties.
	pub fn clear(&mut self) {
		self.props.clear();
	}
	
	///Get a property value giving its name. Return None if property does not exist.
	pub fn find<'a>(&'a self, key : &String) -> Option<&'a String> {
		self.props.find(key)
	}
	
	///Return true if a property value exists for the specified key
	pub fn contains_key(&self, key: &String) -> bool {
		self.props.contains_key(key)
	}
	
	///Insert a property into the list. If the property already had a value present in the list, that value is returned.
	///Otherwise None is returned.
	pub fn swap(&mut self, key: String, value: String) -> Option<String> {
		self.props.swap(key, value)
	}

	///Removes a property from the list, returning the value of the property if it was previously in the list.
	pub fn pop(&mut self, key: &String) -> Option<String> {
		self.props.pop(key)
	}

	///Return a mutable reference to the value corresponding to the property
	pub fn find_mut<'a>(&'a mut self, key: &String) -> Option<&'a mut String> {
		self.props.find_mut(key)
	}

	///Insert a property into the list. An existing value for a property is replaced by the new value.
	///Return true if the property did not already exist in the list.
	pub fn insert(&mut self, key: String, value: String) -> bool {
		self.props.insert(key, value)
	}

	/// Remove a property from the list. Return true if the property was present in the list, otherwise false.
	pub fn remove(&mut self, key: &String) -> bool {
		self.props.remove(key)
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
		let mut multi = String::new();
		for line in BufferedReader::new(reader).lines() {
			match line {
				Ok(l) => {
					multi.push_str(l.as_slice().trim_left());
					if multi.as_slice().starts_with("#") || multi.as_slice().starts_with("!") { multi.clear(); continue; } // Comment line

					if multi.as_slice().ends_with("\n") {
						if multi.as_slice().ends_with("\r\n") { multi=multi.as_slice().slice_to(multi.len()-2).into_string(); } // Line ends with '\r\n'
							else { multi=multi.as_slice().slice_to(multi.len()-1).into_string(); } // Line ends with '\n'
					}
					if multi.len()==0 { multi.clear(); continue; } //Empty line
					
					// line finishing with an odd number of '\' is a multiline
					let mut esc = false;
					for c in multi.as_slice().chars().rev() {
						if c=='\\' { esc=!esc; } else { break; }
					}
					if esc { multi = multi.as_slice().slice_to(multi.len()-1).to_string(); continue; }

					// determination of the key
					esc=false;
					let mut idx = 0u;							
					for c in multi.as_slice().chars() {
						if c=='\\' { esc=true; idx+=1; continue; }
						if !esc && (c.is_whitespace() || c=='=' || c==':') { break; } 
						esc=false; idx+=c.len_utf8_bytes();
					}
					let key = multi.as_slice().slice_to(idx).into_string();
					
					multi = multi.as_slice().slice_from(idx).to_string(); multi = multi.as_slice().trim_left().to_string();
					if multi.as_slice().starts_with("=") || multi.as_slice().starts_with(":") { 	multi=multi.as_slice().slice_from(1).into_string();
																			multi = multi.as_slice().trim_left().into_string(); }					
					self.props.insert(decode_chars(key.as_slice()), decode_chars(multi.as_slice()));
				}
				Err(e) => { return Some(e); }
			}
			multi.clear();
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
				(k,v) => {  let mut line = String::from_str(encode_chars(k.as_slice(), true).as_slice());
							line.push('=');
							line.push_str(encode_chars(v.as_slice(), false).as_slice());
							match buf.write_line(line.as_slice()) 	{ 	Ok(_)  => continue,
															Err(e) => { return Some(e); }
														}
						}
			}
		}
		None
	}

	///An iterator visiting all properties keys in arbitrary order. Iterator element type is &'a String.
	pub fn keys<'a>(&'a self) -> Keys<'a, String, String> {
		self.props.keys()
	}
	
	///An iterator visiting all properties values in arbitrary order. Iterator element type is &'a String.
	pub fn values<'a>(&'a self) -> Values<'a, String, String> {
		self.props.values()
	}

	///An iterator visiting all properties key-value pairs in arbitrary order. Iterator element type is (&'a String, &'a String).
	pub fn iter<'a>(&'a self) -> Entries<'a, String, String> {
		self.props.iter()
	}
	
	///An iterator visiting all properties key-value pairs in arbitrary order, with mutable references to the values.
	///Iterator element type is (&'a String, &'a mut String).
	pub fn iter_mut<'a>(&'a mut self) -> MutEntries<'a, String, String> {
		self.props.iter_mut()
	}
}

fn decode_chars(s : &str) -> String {
	let mut buf = String::from_str(s);
	let mut esc=false;
	let mut idx = 0u;
	for mut c in s.chars() {
		if esc {
			match c {
				't' => { buf.remove(idx); buf.remove(idx); buf.insert(idx, '\t'); esc=false; }
				'f' => { buf.remove(idx); buf.remove(idx); buf.insert(idx, '\x0c'); esc=false; }
				'r' => { buf.remove(idx); buf.remove(idx); buf.insert(idx, '\r'); esc=false; }
				'n' => { buf.remove(idx); buf.remove(idx); buf.insert(idx, '\n'); esc=false; }
				'\\'=> { buf.remove(idx); c=' '; esc=false; }
				 _  => { buf.remove(idx); esc=false; }
			}
		}
		if c=='\\' { esc=true; continue; }
		idx+=c.len_utf8_bytes();
	}
	buf
}

fn encode_chars<'a>(s : &str, is_key : bool) -> String {
	let mut buf = String::from_str(s);
	let mut esc=true;
	let mut idx = 0u;
	for c in s.chars() {
		if c.is_whitespace() 	{
			if esc 	{ 	match c {
							'\t'   => { buf.remove(idx); buf.insert(idx, 't'); buf.insert(idx, '\\'); idx+=1; }
							'\x0c' => { buf.remove(idx); buf.insert(idx, 'f'); buf.insert(idx, '\\'); idx+=1; }
							'\r'   => { buf.remove(idx); buf.insert(idx, 'r'); buf.insert(idx, '\\'); idx+=1; }
							'\n'   => { buf.remove(idx); buf.insert(idx, 'n'); buf.insert(idx, '\\'); idx+=1; }
							_      => { buf.insert(idx, '\\'); idx+=1; }
						}
			} else 	{ 	match c {
							'\r' => { buf.remove(idx); buf.insert(idx, 'r'); buf.insert(idx, '\\'); idx+=1; }
							'\n' => { buf.remove(idx); buf.insert(idx, 'n'); buf.insert(idx, '\\'); idx+=1; }
							_    => ()
						}
					}
		}
		else 	{	if c=='\\' { buf.insert(idx, '\\'); idx+=1; }
					if !is_key { esc=false; }
					else if c=='=' || c==':' { buf.insert(idx, '\\'); idx+=1; }
				}
		idx+=c.len_utf8_bytes();
	}
	if is_key && (s.starts_with("#") || s.starts_with("!")) {
		buf.insert(0, '\\');
	}
	buf
}