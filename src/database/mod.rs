use std::collections::HashMap;

use serde_json::{json, Map, Value};

pub struct Database {
	pub data: HashMap<String, serde_json::Value>,
}

impl Database {
	pub fn new() -> Self {
		let mut database = HashMap::new();
		let v: serde_json::Value = json!([
			{
				"family": "Ana",
				"name": "A",
				"scene": "T",
				"car": {
					"name": "Tesla",
					"speed": "1000"
				}
			},
			{
				"family": "Banana",
				"name": "B",
				"scene": "Y",
				"car": {
					"name": "Tuscon",
					"speed": "2000"
				}
			},
			{
				"family": "Ana",
				"name": "C",
				"scene": "Z",
				"car": {
					"name": "Jaguar",
					"speed": "3000"
				}
			}
		]);

		database.insert("test".to_string(), v);
		Self { data: database }
	}

	pub fn query(&self, command: &str) -> serde_json::Value {
		// Tokenize
		let (get_statement, from_statement) = tokenize_input(command);

		// Parse GET and FROM statements
		let mut from_args = parse_from_statement(&from_statement);
		let mut get_args = parse_get_statement(&get_statement, Some(from_args[0].file));

		// Check that we have some data there
		assert!(!get_args.is_empty());
		assert_eq!(from_args.len(), 1);

		// Prepare for query
		let query_data = parse_args(get_args);

		for (file, fields) in query_data {
			// Read file
			let file_as_json = self.data.get(file).unwrap();
			let file_as_json = file_as_json.as_array().unwrap();

			// Extract the fields that we need
			let res = extract_json_fields(file_as_json, &fields);
			return Value::Array(res)
		}

		todo!()
	}
}

pub fn tokenize_input(inp: &str) -> (Vec<&str>, Vec<&str>) {
	// Lets start with a simple algorithm that only understands GET and FROM
	// First let's tokenize all words in the command
	// Now let's separate our keywords and their arguments

	let mut command = inp.trim();
	let mut tokens: Vec<&str> = command.split(' ').collect();

	// Now let's separate our keywords and their arguments
	let mut get_statement: Vec<&str> = Vec::new();
	let mut from_statement: Vec<&str> = Vec::new();

	let mut curret_statement = &mut get_statement;
	for token in tokens {
		match token {
			"GET" => curret_statement = &mut get_statement,
			"FROM" => curret_statement = &mut from_statement,
			_ => (),
		};

		if token == "GET" || token == "FROM" {
			continue
		}

		curret_statement.push(token);
	}

	(get_statement, from_statement)
}

#[derive(Debug, Clone)]
pub struct GetArgument<'l> {
	pub op: &'l str,
	pub file: &'l str,
	pub field: EntryField<'l>,
	pub field_name: &'l str,
}

impl<'l> GetArgument<'l> {
	pub fn from_str(inp: &'l str, default_file: Option<&'l str>) -> Self {
		let op = inp.clone();
		let all = inp == "ALL";
		let mut field_name = None;

		// Define file name

		// Get the file name if it is there
		let pos = inp.find(':');
		let mut file = match pos {
			Some(pos) => Some(&inp[0..pos]),
			_ => None,
		};

		if file.is_some() && default_file.is_some() {
			assert!(file == default_file);
		}

		if file.is_none() {
			assert!(default_file.is_some());
			file = default_file;
		}
		let file = file.unwrap();

		// Parse field
		let field = if let Some(pos) = pos {
			field_name = Some(&inp[pos + 1..]);
			EntryField::from_str(&inp[pos + 1..])
		} else {
			field_name = Some(&inp);
			EntryField::from_str(inp)
		};

		let field_name = field_name.unwrap();

		Self { op, file, field, field_name }
	}
}

#[derive(Debug, Clone)]
pub enum EntryField<'l> {
	Normal(&'l str),
	Nested((&'l str, Box<EntryField<'l>>)),
	All,
}

impl<'l> EntryField<'l> {
	pub fn from_str(inp: &'l str) -> Self {
		if inp == "ALL" {
			return Self::All
		}

		let dot_pos = inp.find('.');
		if dot_pos.is_none() {
			return Self::Normal(inp)
		}

		let dot_pos = dot_pos.unwrap();
		let parent: &str = &inp[0..dot_pos];
		let child: &str = &inp[dot_pos + 1..];

		Self::Nested((parent, Box::new(EntryField::from_str(child))))
	}
}

pub fn parse_get_statement<'l>(
	statement: &[&'l str],
	file: Option<&'l str>,
) -> Vec<GetArgument<'l>> {
	statement.iter().map(|x| GetArgument::from_str(*x, file)).collect()
}

#[derive(Debug, Clone, Copy)]
pub struct FromArgument<'l> {
	pub file: &'l str,
}

impl<'l> FromArgument<'l> {
	pub fn from_str(inp: &'l str) -> Self {
		Self { file: inp }
	}
}

pub fn parse_from_statement<'l>(statement: &[&'l str]) -> Vec<FromArgument<'l>> {
	statement.iter().map(|x| FromArgument::from_str(*x)).collect()
}

pub fn parse_args<'l>(args: Vec<GetArgument<'l>>) -> HashMap<&'l str, Vec<GetArgument<'l>>> {
	let mut map: HashMap<&'l str, Vec<GetArgument<'l>>> = HashMap::new();

	for arg in args {
		if let Some(entries) = map.get_mut(arg.file) {
			entries.push(arg.clone());
		} else {
			map.insert(arg.file, vec![arg.clone()]);
		}
	}

	map
}

pub fn extract_json_fields<'l>(
	inp: &[serde_json::Value],
	args: &[GetArgument<'l>],
) -> Vec<serde_json::Value> {
	let mut result = Vec::new();

	for inp in inp {
		let mut new_obj: Map<String, Value> = Map::new();
		for arg in args {
			let extracted_field = extract_json_value(inp, &arg.field);

			// Get file name + file name
			let mut full_field_name = std::format!("{}:{}", arg.file, arg.field_name);

			new_obj.insert(full_field_name, extracted_field);
		}
		result.push(Value::Object(new_obj));
	}

	result
}

pub fn extract_json_value<'l>(
	inp: &serde_json::Value,
	field: &EntryField<'l>,
) -> serde_json::Value {
	let inp = inp.as_object().unwrap();

	match field {
		EntryField::Normal(x) => inp.get(*x).unwrap().clone(),
		EntryField::Nested((x, y)) => {
			let parent = inp.get(*x).unwrap();
			extract_json_value(parent, y)
		},
		EntryField::All => todo!(),
	}
}
