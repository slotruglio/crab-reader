use druid::{Color, FontDescriptor, FontFamily };
use serde_json;

#[derive(Debug)]
pub struct MyEnv {
	pub theme: String,
	pub font_color: Color,
	pub font: FontDescriptor,
}

impl MyEnv {
	pub fn new() -> Self {

		let mut new_env: Self = Self {
			theme: "dark".to_string(),
			font_color: Color::rgb8(0, 0, 0),
			font: FontDescriptor::new(FontFamily::SYSTEM_UI),
		};

		//Take the JSON, turn it into a MAP
		let json = std::fs::read_to_string("env.json").unwrap();
		let json: serde_json::Value = serde_json::from_str(&json).unwrap();
		let json = json.as_object().unwrap();

		//SET theme, font_color
		new_env.theme = json.get("theme").unwrap().as_str().unwrap().to_string();
		new_env.font_color = MyEnv::get_color(json.get("font_color").unwrap().to_string());

		//SET font_size, font_family
		let font_size_string = json.get("font_size").unwrap().as_str().unwrap();
		let font_size_numeric: f64;

		match font_size_string {
			"small" => font_size_numeric = 12.0,
			"medium" => font_size_numeric = 16.0,
			"large" => font_size_numeric = 20.0,
			_ => font_size_numeric = 16.0,
		}

		new_env.font = FontDescriptor::new(MyEnv::get_font_family(json.get("font_family").unwrap().to_string()))
			.with_size(font_size_numeric);

		return new_env;
	}

	pub fn save_to_env(&mut self) {
		//open a new file, creating it if it doesn't exist
		let file = std::fs::File::create("env.json").unwrap();

		//create a new json object
		let mut json = serde_json::Map::new();

		//add the values to the json object
		json.insert("theme".to_string(), serde_json::Value::String(self.theme.clone()));
		json.insert("font_color".to_string(), serde_json::Value::String(MyEnv::get_color_reverse(self.font_color.clone())));
		json.insert("font_size".to_string(), serde_json::Value::String(MyEnv::get_font_size_reverse(self.font.size)));
		json.insert("font_family".to_string(), serde_json::Value::String(MyEnv::get_font_family_reverse(self.font.family.clone())));

		//write the json object to the file
		serde_json::to_writer_pretty(file, &json).unwrap();
	}

	pub fn set_property(&mut self, property: String, value: String) {
		match property.as_str() {
			"theme" => self.theme = value,
			"font_color" => self.font_color = MyEnv::get_color(value),
			"font_size" => self.font = FontDescriptor::new(self.font.family.clone()).with_size(MyEnv::get_font_size(value)),
			"font_family" => self.font = FontDescriptor::new(MyEnv::get_font_family(value)).with_size(self.font.size),
			_ => (),
		}
	}

	//HELPER METHODS
	fn get_color(value: String) -> druid::Color {
		//match value against every color contained in the Color struct
		match value.as_str() {
			"BLACK" => druid::Color::BLACK,
			"NAVY" => druid::Color::NAVY,
			"WHITE" => druid::Color::WHITE,
			"TEAL" => druid::Color::TEAL,
			_ => druid::Color::BLACK
		}
	}

	fn get_color_reverse(value: druid::Color) -> String {
		//match value against every color contained in the Color struct
		if value == druid::Color::BLACK {
			return "BLACK".to_string();
		} else if value == druid::Color::NAVY {
			return "NAVY".to_string();
		} else if value == druid::Color::WHITE {
			return "WHITE".to_string();
		} else if value == druid::Color::TEAL {
			return "TEAL".to_string();
		} else {
			return "BLACK".to_string();
		}
	}

	fn get_font_family(value: String) -> FontFamily {
		match value.as_str() {
			"MONOSPACE" => FontFamily::MONOSPACE,
			"SYSTEM_UI" => FontFamily::SYSTEM_UI,
			"SERIF" => FontFamily::SERIF,
			"SANS_SERIF" => FontFamily::SANS_SERIF,
			_ => FontFamily::SYSTEM_UI
		}
	}

	fn get_font_family_reverse(value: FontFamily) -> String {
		match value {
			FontFamily::MONOSPACE => "MONOSPACE".to_string(),
			FontFamily::SYSTEM_UI => "SYSTEM_UI".to_string(),
			FontFamily::SERIF => "SERIF".to_string(),
			FontFamily::SANS_SERIF => "SANS_SERIF".to_string(),
			_ => "SYSTEM_UI".to_string()
		}
	}

	fn get_font_size(value: String) -> f64 {
		match value.as_str() {
			"small" => 12.0,
			"medium" => 16.0,
			"large" => 20.0,
			_ => 16.0,
		}
	}

	fn get_font_size_reverse(value: f64) -> String {

		if value == 12.0 {
			return "small".to_string();
		} else if value == 16.0 {
			return "medium".to_string();
		} else if value == 20.0 {
			return "large".to_string();
		} else {
			return "medium".to_string();
		}
	}

}