use std::collections::HashMap;

use serde_json::{json, Map, Value};

/* pub struct QueryStateMachine();



pub trait State {
	pub fn execute()
}

 */

pub struct FromArgument<'l> {
	pub file: &'l str,
}

impl<'l> FromArgument<'l> {
	pub fn from_str(inp: &'l str) -> Self {
		Self { file: inp }
	}
}

pub struct GetArgument<'l> {
	pub op: &'l str,
	pub file: Option<&'l str>,
	pub field: Option<&'l str>,
	pub all: bool,
}

impl<'l> GetArgument<'l> {
	pub fn from_str(inp: &'l str) -> Self {
		let op = inp.clone();
		let all = inp == "ALL";

		// Get the file name if it is there
		let file = match inp.find(':') {
			Some(pos) => Some(&inp[0..pos]),
			_ => None,
		};

		let field = match inp.find(':') {
			Some(pos) => Some(&inp[pos..]),
			_ => Some(inp),
		};

		Self { op, file, field, all }
	}
}

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
				"scene": "T"
			},
			{
				"family": "Banana",
				"name": "B",
				"scene": "Y"
			},
			{
				"family": "Ana",
				"name": "C",
				"scene": "Z"
			}
		]);

		database.insert("test".to_string(), v);
		Self { data: database }
	}

	pub fn query(&self, command: &str) -> serde_json::Value {
		// Lets start with a simple algorithm that only understands GET and FROM
		// First let's tokenize all words in the command
		let mut command = command.trim();
		let mut tokens: Vec<&str> = command.split(' ').collect();

		// Now let's separate our keywords and their arguments
		let mut get_command: Vec<&str> = Vec::new();
		let mut from_command: Vec<&str> = Vec::new();

		let mut curret_comamnd = &mut get_command;
		for token in tokens {
			match token {
				"GET" => curret_comamnd = &mut get_command,
				"FROM" => curret_comamnd = &mut from_command,
				_ => (),
			};

			if token == "GET" || token == "FROM" {
				continue
			}

			curret_comamnd.push(token);
		}

		// If GET or FROM are empty return
		if get_command.is_empty() || from_command.is_empty() {
			todo!()
		}

		// Parse all GET Arguments
		let mut get_arguments: Vec<GetArgument> =
			get_command.iter().map(|x| GetArgument::from_str(*x)).collect();

		// Parse all FROM Arguments
		let from_arguments: Vec<FromArgument> =
			from_command.iter().map(|x| FromArgument::from_str(*x)).collect();

		// Let's just deal with 1 FROM
		assert_eq!(from_arguments.len(), 1);

		// Since FROM is just one then we can apply this from to all get arguments
		get_arguments.iter_mut().for_each(|x| {
			x.file = Some(from_arguments[0].file);
		});

		// If we have an ALL keyword then we can ignore all the other arguments
		let mut all = get_arguments.iter().find(|x| x.all).is_some();

		let mut file_to_field_map = HashMap::<&str, Vec<&str>>::new();
		for from_argument in from_arguments {
			let file = from_argument.file;

			if all {
				file_to_field_map.insert(file, vec![]);
				break
			}

			let valid_args = get_arguments.iter().filter(|x| x.file == Some(file));
			for arg in valid_args {
				let field = arg.field.unwrap();
				if let Some(x) = file_to_field_map.get_mut(file) {
					x.push(field);
				} else {
					file_to_field_map.insert(file, vec![field]);
				}
			}
		}

		let mut file_to_json_output = HashMap::<&str, serde_json::Value>::new();
		for (file, fields) in file_to_field_map {
			let read_file = self.data.get(file).unwrap();

			if fields.is_empty() {
				file_to_json_output.insert(file, read_file.clone());
				continue
			}

			let entires = read_file.as_array().unwrap();
			let mut new_entires: Vec<Value> = Vec::new();
			for entry in entires {
				let old_obj = entry.as_object().unwrap();
				let mut new_obj: Map<String, Value> = Map::new();

				for field in fields.iter() {
					let field = field.to_string();
					new_obj.insert(field.clone(), old_obj.get(&field).unwrap().clone());
				}

				new_entires.push(Value::Object(new_obj));
			}

			file_to_json_output.insert(file, Value::Array(new_entires));
		}

		// Let's see what kind of OP we have for GET

		file_to_json_output.get("test").unwrap().clone()

		/* 		let mut multi_file_mode = false;

		// Check if we are dealing with one file or multiple files
		let mut from_token_pos = tokens.iter().position(|x| **x == "FROM").unwrap();
		let mut next_keyword_pos = tokens.iter()

		let token = tokens.pop().unwrap();
		assert_eq!(token, "GET");

		let token = tokens.pop().unwrap();
		assert_eq!(token, "ALL");

		let token = tokens.pop().unwrap();
		assert_eq!(token, "FROM");

		let file_name = tokens.pop().unwrap();
		let db_value = self.data.get(file_name).unwrap();
		let db_value = db_value.as_array().unwrap();

		let token = tokens.pop().unwrap();
		assert_eq!(token, "WHERE");

		let field = tokens.pop().unwrap();

		let token = tokens.pop().unwrap();
		assert_eq!(token, "=");

		let expected_value = tokens.pop().unwrap();

		let all_items: Vec<serde_json::Value> = db_value
			.iter()
			.filter(|x| {
				let value = x.as_object().unwrap();
				let field_value = value.get(field).unwrap();
				field_value.as_str().unwrap() == expected_value
			})
			.map(|x| x.clone())
			.collect();

		serde_json::Value::Array(all_items) */
	}
}
