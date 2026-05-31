use std::{fmt, path::Path};

use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ConfigError {
    Validation(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Application configuration read at startup from config.toml file
///
/// If no toml file is provided, applies the default values in the configuration
///
/// Configuration values include:
/// * `iterations` - The number of iterations to run the local search algorithm
/// * `number_of_global_searches` - The number of global search algorithms to run in parallel
/// * `constraint_file_path` - The path to the constraints.json file
/// * `initial_temperature` - The initial temperature for the local search algorithm
/// * `cooling_factor` - The cooling factor for the local search algorithm
/// * `random_seed` - The random seed used for generating quasi random schedules
/// * `output_path` - The path to output the generated schedule to, include the name of the output
/// csv file as well (e.g. ../schedule.csv)
/// * `debug` - Whether to run in debug mode or not
///
/// TODO: Add the penalty values
/// TODO: Add the chances for optimisation strategy choosing
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub iterations: u32,
    pub number_of_global_searches: u32,
    pub constraint_file_path: String,
    pub initial_temperature: f32,
    pub cooling_factor: f32,
    pub random_seed: u32,
    pub output_path: String,
    pub debug: bool,
}

impl Default for Config {
    /// The default values for the configuration
    fn default() -> Config {
        Config {
            iterations: 1000,
            number_of_global_searches: 5,
            constraint_file_path: "../constraints.json".to_string(),
            initial_temperature: 200.0,
            cooling_factor: 0.999,
            random_seed: 0,
            output_path: "../".to_string(),
            debug: true,
        }
    }
}

impl Config {
    pub fn new() -> Result<Config, ConfigError> {
        let config: Config = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file("config.toml"))
            .extract()
            .unwrap();

        config.validate()?;
        Ok(config)
    }

    // TODO: Add more validation logic
    fn validate(&self) -> Result<(), ConfigError> {
        if Path::new(&self.output_path).file_name().is_none() {
            return Err(ConfigError::Validation("The specified output directory is invalid. Please ensure the file name is also specified".to_string()));
        }

        Ok(())
    }
}
