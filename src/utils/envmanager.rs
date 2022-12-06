use std::path::PathBuf;

use druid::{Color, FontDescriptor, FontFamily};
use serde_json::{self, json};

use super::{fonts, dir_manager::get_env_path};

#[derive(Debug)]
pub struct MyEnv {
    pub theme: String,
    pub font_color: Color,
    pub font: FontDescriptor,
    pub shadows: bool,
}

impl MyEnv {
    pub fn new(env_path: PathBuf) -> Self {
        let mut new_env: Self = Self {
            theme: "light".to_string(),
            font_color: Color::rgb8(0, 0, 0),
            font: FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(FontSize::MEDIUM.to_f64()),
            shadows: false,
        };

        //Take the JSON, turn it into a MAP
        let Ok(json) = std::fs::read_to_string(&env_path) else {
            //If the file doesn't exist, create it
            let file = std::fs::File::create(env_path).unwrap();
            let json = json!(
                {
                    "font_color": "WHITE",
                    "font_family": "SISTEM_UI",
                    "font_size": "medium",
                    "theme": "light",
                    "shadows": false
                }
            );
            let _ = serde_json::to_writer_pretty(file, &json);
            return new_env;
        };
        let json: serde_json::Value = serde_json::from_str(&json).unwrap();
        let json = json.as_object().unwrap();

        //SET theme, font_color
        new_env.theme = json.get("theme").unwrap().as_str().unwrap().to_string();
        new_env.font_color = MyEnv::get_color(json.get("font_color").unwrap().to_string());

        //SET font_size, font_family
        let font_size_string = json.get("font_size").unwrap().as_str().unwrap();
        
        let font_size_numeric: f64 = FontSize::from(font_size_string).to_f64();

        new_env.font = FontDescriptor::new(MyEnv::get_font_family(
            json.get("font_family").unwrap().to_string(),
        ))
        .with_size(font_size_numeric);

        new_env.shadows = json.get("shadows").unwrap().as_bool().unwrap();

        return new_env;
    }

    #[allow(dead_code)]
    pub fn save_to_env(&mut self) {
        //open a new file, creating it if it doesn't exist
        let file = std::fs::File::create(get_env_path()).unwrap();

        //create a new json object
        let mut json = serde_json::Map::new();

        //add the values to the json object
        json.insert(
            "theme".to_string(),
            serde_json::Value::String(self.theme.clone()),
        );
        json.insert(
            "font_color".to_string(),
            serde_json::Value::String(MyEnv::get_color_reverse(self.font_color.clone())),
        );
        json.insert(
            "font_size".to_string(),
            serde_json::Value::String(MyEnv::get_font_size_reverse(self.font.size)),
        );
        json.insert(
            "font_family".to_string(),
            serde_json::Value::String(MyEnv::get_font_family_reverse(self.font.family.clone())),
        );
        json.insert(
            "shadows".to_string(),
            serde_json::Value::Bool(self.shadows.clone()),
        );

        //write the json object to the file
        serde_json::to_writer_pretty(file, &json).unwrap();
    }

    #[allow(dead_code)]
    pub fn set_property(&mut self, property: String, value: String) {
        match property.as_str() {
            "theme" => self.theme = value,
            "font_color" => self.font_color = MyEnv::get_color(value),
            "font_size" => {
                self.font = FontDescriptor::new(self.font.family.clone())
                    .with_size(MyEnv::get_font_size(value))
            }
            "font_family" => {
                self.font =
                    FontDescriptor::new(MyEnv::get_font_family(value)).with_size(self.font.size)
            }
            "shadows" => self.shadows = value.parse::<bool>().unwrap(),
            _ => (),
        }
    }

    //HELPER METHODS
    fn get_color(value: String) -> druid::Color {

        //If value is surrounded by quotes, remove them
        let value = value.replace("\"", "");

        //match value against every color contained in the Color struct
        match value.as_str() {
            "BLACK" => druid::Color::BLACK,
            "NAVY" => druid::Color::NAVY,
            "WHITE" => druid::Color::WHITE,
            "TEAL" => druid::Color::TEAL,
            _ => druid::Color::BLACK,
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

        //If value is surrounded by quotes, remove them
        let value = value.replace("\"", "");

        match value.as_str() {
            "MONOSPACE" => FontFamily::MONOSPACE,
            "SYSTEM_UI" => FontFamily::SYSTEM_UI,
            "SERIF" => FontFamily::SERIF,
            "SANS_SERIF" => FontFamily::SANS_SERIF,
            _ => FontFamily::SYSTEM_UI,
        }
    }

    fn get_font_family_reverse(value: FontFamily) -> String {
        match value {
            FontFamily::MONOSPACE => "MONOSPACE".to_string(),
            FontFamily::SYSTEM_UI => "SYSTEM_UI".to_string(),
            FontFamily::SERIF => "SERIF".to_string(),
            FontFamily::SANS_SERIF => "SANS_SERIF".to_string(),
            _ => "SYSTEM_UI".to_string(),
        }
    }

    fn get_font_size(value: String) -> f64 {
        FontSize::from(value).to_f64()
    }

    fn get_font_size_reverse(value: f64) -> String {
        FontSize::from(value).to_string()
    }
}

pub enum FontSize {
    SMALL,
    MEDIUM,
    LARGE,
}
impl FontSize {
    pub fn to_f64(&self) -> f64 {
        match self {
            FontSize::SMALL => fonts::small.size,
            FontSize::MEDIUM => fonts::medium.size,
            FontSize::LARGE => fonts::large.size,
        }
    }

}

impl PartialEq for FontSize {
    fn eq(&self, other: &Self) -> bool {
        self.to_f64() == other.to_f64()
    }
}

impl From<f64> for FontSize {
    fn from(value: f64) -> Self {
        if value == fonts::small.size {
            return FontSize::SMALL;
        } else if value == fonts::medium.size {
            return FontSize::MEDIUM;
        } else if value == fonts::large.size {
            return FontSize::LARGE;
        }
        FontSize::MEDIUM
    }
}
impl From<String> for FontSize {
    fn from(value: String) -> Self {
        match value.as_str() {
            "small" => FontSize::SMALL,
            "medium" => FontSize::MEDIUM,
            "large" => FontSize::LARGE,
            _ => FontSize::MEDIUM,
        }
    }
}
impl From<&str> for FontSize {
    fn from(value: &str) -> Self {
        match value {
            "small" => FontSize::SMALL,
            "medium" => FontSize::MEDIUM,
            "large" => FontSize::LARGE,
            _ => FontSize::MEDIUM,
        }
    }
}
impl ToString for FontSize {
    fn to_string(&self) -> String {
        match self {
            FontSize::SMALL => "small".to_string(),
            FontSize::MEDIUM => "medium".to_string(),
            FontSize::LARGE => "large".to_string(),
        }
    }
}


//TEST METHODS
#[cfg(test)]
mod tests {
    use std::io::Write;

    use serial_test::serial;

    use super::*;

    //MAIN FUNCTIONS TESTS

    #[test]
    #[serial]
    fn test_new_fresh() {
        //create a new MyEnv, passing ./config/env.test.json as a PathBuf parameter
        let env = MyEnv::new(PathBuf::from("./conf/env.test.json"));

        //The file doesnt exist, so the default values should be used
        assert_eq!(env.font, fonts::medium);
        assert_eq!(env.font_color, Color::rgb8(0, 0, 0));
        assert_eq!(env.theme, "light".to_string());
        assert_eq!(env.shadows, false);

        //delete the env.test.json file
        std::fs::remove_file("./conf/env.test.json").unwrap();
    }

    #[test]
    #[serial]
    fn test_new_normal() {

        //create a new env.test.json file
        let mut file = std::fs::File::create("./conf/env.test.json").unwrap();

        //write the json content
        file.write_all(
            r#"
            {
                "font_color": "NAVY",
                "font_family": "MONOSPACE",
                "font_size": "small",
                "shadows": true,
                "theme": "dark"
            }
            "#
            .as_bytes(),
        ).unwrap();


        //create a new MyEnv, passing ./config/env.test.json as a PathBuf parameter
        let env = MyEnv::new(PathBuf::from("./conf/env.test.json"));

        //The file exists, so the values should be the ones contained in the file
        assert_eq!(env.font.family, FontFamily::MONOSPACE);
        assert_eq!(env.font.size, FontSize::SMALL.to_f64());
        assert_eq!(env.font_color, Color::NAVY);
        assert_eq!(env.theme, "dark".to_string());
        assert_eq!(env.shadows, true);

        //delete the env.test.json file
        std::fs::remove_file("./conf/env.test.json").unwrap();

    }


    fn test_save_to_env() {
        todo!();
    }

    #[test]
    #[serial]
    fn test_set_property() {
        //create a new env.test.json file
        let mut file = std::fs::File::create("./conf/env.test.json").unwrap();

        //write the json content
        file.write_all(
            r#"
            {
                "font_color": "NAVY",
                "font_family": "MONOSPACE",
                "font_size": "small",
                "shadows": true,
                "theme": "dark"
            }
            "#
            .as_bytes(),
        ).unwrap();

        //create a new MyEnv, passing ./config/env.test.json as a PathBuf parameter
        let mut env = MyEnv::new(PathBuf::from("./conf/env.test.json"));

        //set the font family to serif
        env.set_property("font_family".to_string(), "SERIF".to_string());
        assert_eq!(env.font.family, FontFamily::SERIF);

        //set the font size to large
        env.set_property("font_size".to_string(), "large".to_string());
        assert_eq!(env.font.size, FontSize::LARGE.to_f64());

        //set the font color to white
        env.set_property("font_color".to_string(), "WHITE".to_string());
        assert_eq!(env.font_color, Color::WHITE);

        //set the theme to light
        env.set_property("theme".to_string(), "light".to_string());
        assert_eq!(env.theme, "light".to_string());

        //set the shadows to false
        env.set_property("shadows".to_string(), "false".to_string());
        assert_eq!(env.shadows, false);



        //delete the env.test.json file
        std::fs::remove_file("./conf/env.test.json").unwrap();

    }


    //HELPER METHODS TESTS

    #[test]
    fn test_get_color() {
        assert_eq!(MyEnv::get_color("\"BLACK\"".to_string()), druid::Color::BLACK);
        assert_eq!(MyEnv::get_color("\"NAVY\"".to_string()), druid::Color::NAVY);
        assert_eq!(MyEnv::get_color("\"WHITE\"".to_string()), druid::Color::WHITE);
        assert_eq!(MyEnv::get_color("\"TEAL\"".to_string()), druid::Color::TEAL);
    }

    #[test]
    fn test_get_color_reverse() {
        assert_eq!(MyEnv::get_color_reverse(druid::Color::BLACK), "BLACK".to_string());
        assert_eq!(MyEnv::get_color_reverse(druid::Color::NAVY), "NAVY".to_string());
        assert_eq!(MyEnv::get_color_reverse(druid::Color::WHITE), "WHITE".to_string());
        assert_eq!(MyEnv::get_color_reverse(druid::Color::TEAL), "TEAL".to_string());
    }

    #[test]
    fn test_get_font_family() {
        assert_eq!(
            MyEnv::get_font_family("\"MONOSPACE\"".to_string()),
            FontFamily::MONOSPACE
        );
        assert_eq!(
            MyEnv::get_font_family("\"SYSTEM_UI\"".to_string()),
            FontFamily::SYSTEM_UI
        );
        assert_eq!(MyEnv::get_font_family("\"SERIF\"".to_string()), FontFamily::SERIF);
        assert_eq!(
            MyEnv::get_font_family("\"SANS_SERIF\"".to_string()),
            FontFamily::SANS_SERIF
        );
    }

    #[test]
    fn test_get_font_family_reverse() {
        assert_eq!(
            MyEnv::get_font_family_reverse(FontFamily::MONOSPACE),
            "MONOSPACE".to_string()
        );
        assert_eq!(
            MyEnv::get_font_family_reverse(FontFamily::SYSTEM_UI),
            "SYSTEM_UI".to_string()
        );
        assert_eq!(
            MyEnv::get_font_family_reverse(FontFamily::SERIF),
            "SERIF".to_string()
        );
        assert_eq!(
            MyEnv::get_font_family_reverse(FontFamily::SANS_SERIF),
            "SANS_SERIF".to_string()
        );
    }

    #[test]
    fn test_get_font_size() {
        assert_eq!(MyEnv::get_font_size("small".to_string()), fonts::small.size);
        assert_eq!(MyEnv::get_font_size("medium".to_string()), fonts::medium.size);
        assert_eq!(MyEnv::get_font_size("large".to_string()), fonts::large.size);
    }

    #[test]
    fn test_get_font_size_reverse() {
        assert_eq!(MyEnv::get_font_size_reverse(fonts::small.size), "small".to_string());
        assert_eq!(MyEnv::get_font_size_reverse(fonts::medium.size), "medium".to_string());
        assert_eq!(MyEnv::get_font_size_reverse(fonts::large.size), "large".to_string());
    }

    //TRAIT METHODS TESTS

    
}
