use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path;

use toml;

#[derive(Debug)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub population_size: u32,
    pub alpha: u8,

    pub add_polygon_chance: f32,
    pub remove_polygon_chance: f32,
    pub move_polygon_chance: f32,
    pub alter_polygon_color_chance: f32,

    pub max_move_amount: f32,
}

impl Config {
    pub fn from_config_file<T>(as_path: T) -> Config where T: AsRef<path::Path> {
        let path = as_path.as_ref();
        let mut file = match fs::File::open(&path) {
            Err(why) => panic!("Couldn't open the configuration file ({:?}): {}", path, why.description()),
            Ok(file) => file,
        };

        let mut file_contents = String::new();
        if let Err(why) = file.read_to_string(&mut file_contents) {
            panic!("Couldn't read from the configuration file: {}", why.description());
        };

        let toml_value = toml::Parser::new(&file_contents).parse().unwrap();

        let width = Config::get_int(&toml_value, "width");
        let height = Config::get_int(&toml_value, "height");
        let population_size = Config::get_int(&toml_value, "population_size");
        let alpha = Config::get_int(&toml_value, "alpha") as u8;

        let add_polygon_chance = Config::get_float(&toml_value, "add_polygon_chance");
        let remove_polygon_chance = Config::get_float(&toml_value, "add_polygon_chance");
        let move_polygon_chance = Config::get_float(&toml_value, "move_polygon_chance");
        let alter_polygon_color_chance = Config::get_float(&toml_value, "alter_polygon_color_chance");

        let max_move_amount = Config::get_float(&toml_value, "max_move_amount");

        Config {
            width,
            height,
            population_size,
            alpha,

            add_polygon_chance,
            remove_polygon_chance,
            move_polygon_chance,
            alter_polygon_color_chance,

            max_move_amount,
        }
    }

    fn get_int(toml_value: &BTreeMap<String, toml::Value>, parameter: &'static str) -> u32 {
        toml_value.get(parameter)
            .expect(&format!("{} configuration not specified", parameter))
            .as_integer()
            .expect(&format!("{} configuration was incorrect", parameter))
            as u32
    }

    fn get_float(toml_value: &BTreeMap<String, toml::Value>, parameter: &'static str) -> f32 {
        toml_value.get(parameter)
            .expect(&format!("{} configuration not specified", parameter))
            .as_float()
            .expect(&format!("{} configuration was incorrect", parameter))
            as f32
    }
}
