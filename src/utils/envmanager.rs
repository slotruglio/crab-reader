use druid::{Env, Key, Color, FontDescriptor, FontFamily, FontStyle, piet::Text};

use serde_json;

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
                if font.is_err() {
                    //if it doesn't exist, create it
                    env.set(
                        Key::<FontDescriptor>::new("font"), 
                        FontDescriptor::new(FontFamily::SYSTEM_UI)
                    );
                } else {
                    //if it exists, update it
                    env.set(
                        Key::<FontDescriptor>::new("font"), 
                        FontDescriptor::new(FontFamily::MONOSPACE).with_size(font.unwrap().size)
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

fn get_style(value: String) -> FontStyle {
    match value.as_str() {
        "ITALIC" => FontStyle::Italic,
        "REGULAR" => FontStyle::Regular,
        _ => FontStyle::Regular
    }
}