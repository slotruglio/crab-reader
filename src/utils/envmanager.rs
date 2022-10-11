use druid::{Env, Key, Color, FontDescriptor, FontFamily, FontStyle};

use serde_json;

use druid::piet::*;

//function that saves the current environment to a json file
pub fn save_env(env: &Env, path: &str) {

	//open a new file, creating it if it doesn't exist
	let file = std::fs::File::create(path).unwrap();

	//create a new json object
	let mut json = serde_json::Map::new();

	//for each key in the environment

	for key in vec!["font".to_string(), "font_color".to_string(), "background_color".to_string()] {
		//get the value of the key
		match key.to_string().as_str() {
			"font_color" => {
				let value = env.get(Key::<Color>::new("font_color"));

				json.insert(
					key.to_string(), 
					serde_json::Value::String(get_color_reverse(value))
            	);
			},
			"font" => {
				let value = env.get(Key::<FontDescriptor>::new("font"));

				json.insert(
					"font_size".to_string(), 
					serde_json::Value::String(value.size.to_string())
				);
				json.insert(
					"font_family".to_string(), 
					serde_json::Value::String(value.family.name().to_string())
				);
				json.insert(
					"font_style".to_string(), 
					serde_json::Value::String(get_style_reverse(value.style))				
				);
			},
			"background_color" => {
				let value = env.get(Key::<Color>::new("background_color"));

				json.insert(
					key.to_string(), 
					serde_json::Value::String(get_color_reverse(value))
				);
			},
			_ => ()
		}	
	}

	//write the json object to the file
	serde_json::to_writer_pretty(file, &json).unwrap();
}


//function that reads a json file and put the keys into a druid::env
pub fn read_env_from_json(env: &mut Env) {
    let json = std::fs::read_to_string("env.json").unwrap();

    let json: serde_json::Value = serde_json::from_str(&json).unwrap();

    let json = json.as_object().unwrap();
    for (key, value) in json {

        match key.as_str() {
            "font_color" => env.set(
                Key::<Color>::new("font_color"), 
                get_color(value.to_string())
            ),
            "background_color" => env.set(
                Key::<Color>::new("background_color"), 
                get_color(value.to_string())
            ),
            "font_size" => {
                    //get environment variable called font
                    let font = env.try_get(Key::<FontDescriptor>::new("font"));
                    if font.is_err() {
                        //if it doesn't exist, create it
                        env.set(
                            Key::<FontDescriptor>::new("font"), 
                            FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(value.as_f64().unwrap() as f64)
                        );
                    } else {
                        //if it exists, update it
                        env.set(
                            Key::<FontDescriptor>::new("font"), 
                            font.unwrap().with_size(value.as_f64().unwrap() as f64)
                        );
                    }  
            },
            "font_family" => {
                //get environment variable called font
                let font = env.try_get(Key::<FontDescriptor>::new("font"));
				let mut ctx = NullRenderContext::new();
				let text = ctx.text();
				let text_font = text.font_family(value.as_str().unwrap())
					.or_else(|| text.font_family(value.as_str().unwrap()))
					.unwrap_or(FontFamily::SYSTEM_UI);

                if font.is_err() {
                    //if it doesn't exist, create it
                    env.set(
                        Key::<FontDescriptor>::new("font"), 
                        FontDescriptor::new(text_font)
                    );
                } else {
                    //if it exists, update it
					

                    env.set(
                        Key::<FontDescriptor>::new("font"), 
                        FontDescriptor::new(text_font).with_size(font.unwrap().size)
                    ); //TODO: SOSTITUIRE MONOSPACE CON LA VERA FAMILY ATTUALE DELLA VARIABILE value AL POSTO DI MONOSPACE

                }  
            },
            "font_style" => {
                //get environment variable called font
                let font = env.try_get(Key::<FontDescriptor>::new("font"));
                if font.is_err() {
                    //if it doesn't exist, create it
                    env.set(
                        Key::<FontDescriptor>::new("font"), 
                        FontDescriptor::new(FontFamily::SYSTEM_UI).with_style(get_style(value.to_string()))
                        
                    );
                } else {
                    //if it exists, update it
                    env.set(
                        Key::<FontDescriptor>::new("font"), 
                        font.unwrap().with_style(get_style(value.to_string()))
                    );
                }  
            },
            _ => ()
        }
        
    }
}

fn get_color(value: String) -> druid::Color {
    //match value against every color contained in the Color struct
    match value.as_str() {
        "AQUA" => druid::Color::AQUA,
        "BLACK" => druid::Color::BLACK,
        "BLUE" => druid::Color::BLUE,
        "FUCHSIA" => druid::Color::FUCHSIA,
        "GRAY" => druid::Color::GRAY,
        "GREEN" => druid::Color::GREEN,
        "LIME" => druid::Color::LIME,
        "MAROON" => druid::Color::MAROON,
        "NAVY" => druid::Color::NAVY,
        "OLIVE" => druid::Color::OLIVE,
        "PURPLE" => druid::Color::PURPLE,
        "RED" => druid::Color::RED,
        "SILVER" => druid::Color::SILVER,
        "TEAL" => druid::Color::TEAL,
        "WHITE" => druid::Color::WHITE,
        "YELLOW" => druid::Color::YELLOW,
        _ => druid::Color::BLACK
    }
}

fn get_color_reverse(value: druid::Color) -> String {
	//match value against every color contained in the Color struct
	if value == druid::Color::AQUA {
		return "AQUA".to_string();
	} else if value == druid::Color::BLACK {
		return "BLACK".to_string();
	} else if value == druid::Color::BLUE {
		return "BLUE".to_string();
	} else if value == druid::Color::FUCHSIA {
		return "FUCHSIA".to_string();
	} else if value == druid::Color::GRAY {
		return "GRAY".to_string();
	} else if value == druid::Color::GREEN {
		return "GREEN".to_string();
	} else if value == druid::Color::LIME {
		return "LIME".to_string();
	} else if value == druid::Color::MAROON {
		return "MAROON".to_string();
	} else if value == druid::Color::NAVY {
		return "NAVY".to_string();
	} else if value == druid::Color::OLIVE {
		return "OLIVE".to_string();
	} else if value == druid::Color::PURPLE {
		return "PURPLE".to_string();
	} else if value == druid::Color::RED {
		return "RED".to_string();
	} else if value == druid::Color::SILVER {
		return "SILVER".to_string();
	} else if value == druid::Color::TEAL {
		return "TEAL".to_string();
	} else if value == druid::Color::WHITE {
		return "WHITE".to_string();
	} else if value == druid::Color::YELLOW {
		return "YELLOW".to_string();
	} else {
		return "BLACK".to_string();
	}
}

fn get_style(value: String) -> FontStyle {
    match value.as_str() {
        "ITALIC" => FontStyle::Italic,
        "REGULAR" => FontStyle::Regular,
        _ => FontStyle::Regular
    }
}

fn get_style_reverse(value: FontStyle) -> String {
	match value {
		FontStyle::Italic => "ITALIC".to_string(),
		FontStyle::Regular => "REGULAR".to_string(),
		_ => "REGULAR".to_string()
	}
}

// fn get_font_family(value: String) -> FontFamily {
// 	match value.as_str() {
// 		"MONOSPACE" => FontFamily::MONOSPACE,
// 		"SYSTEM_UI" => FontFamily::SYSTEM_UI,
// 		"SERIF" => FontFamily::SERIF,
// 		"SANS_SERIF" => FontFamily::SANS_SERIF,
// 		_ => FontFamily::SYSTEM_UI
// 	}
// }

// fn get_font_family_reverse(value: FontFamily) -> String {
// 	match value {
// 		FontFamily::MONOSPACE => "MONOSPACE".to_string(),
// 		FontFamily::SYSTEM_UI => "SYSTEM_UI".to_string(),
// 		FontFamily::SERIF => "SERIF".to_string(),
// 		FontFamily::SANS_SERIF => "SANS_SERIF".to_string(),
// 		_ => "SYSTEM_UI".to_string()
// 	}
// }