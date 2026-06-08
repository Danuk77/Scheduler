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

/// Configuration values for specifying the penalty value for each penalty type used
///
/// Configuration values include:
/// * `presence_high`: The penalty applied for a high priority task when it is not scheduled
/// * `presence_low`: The penalty applied for a low priority task when it is not scheduled
/// * `allowed_slots_high`: The penalty applied for a high priority task when it is not scheduled
///     in an allowed slot
/// * `allowed_slots_low`: The penalty applied for a low priority task when it is not scheduled
///     in an allowed slot
/// * `preferred_slots_high`: The penalty applied for a high priority task when it is not scheduled
///     in a preferred slot
/// * `preferred_slots_low`: The penalty applied for a low priority task when it is not scheduled
///     in a preferred slot
/// * `gap_high`: The penalty applied for a high priority task when it's configured gap constraint
///     is voided
/// * `gap_low`: The penalty applied for a low priority task when it's configured gap constraint
///     is voided
#[derive(Serialize, Deserialize, Debug)]
pub struct PenaltiesConfig {
    pub presence_high: u32,
    pub presence_low: u32,
    pub allowed_slots_high: u32,
    pub allowed_slots_low: u32,
    pub preferred_slots_high: u32,
    pub preferred_slots_low: u32,
    pub gap_high: u32,
    pub gap_low: u32,
}

impl Default for PenaltiesConfig {
    fn default() -> Self {
        PenaltiesConfig {
            presence_high: 10,
            presence_low: 5,
            allowed_slots_high: 30,
            allowed_slots_low: 20,
            preferred_slots_high: 3,
            preferred_slots_low: 2,
            gap_high: 3,
            gap_low: 2,
        }
    }
}

/// Configuration used for specifying the chances for each optimisation strategy within the
/// make_small_chance function
///
/// NOTE: Any positive number can be used for specifying these chances, the final probabilities are
/// identified after noramlisation
///
/// Configuration includes:
/// * `move_chance` - The chance of move optimisation strategy is chosen
/// * `unschedule_chance` - The chance of unschedule optimisation strategy is chosen
/// * `swap_chance` - The chance of swap optimisation strategy is chosen
#[derive(Serialize, Deserialize, Debug)]
pub struct OptimisationStrategyConfig {
    pub move_chance: u32,
    pub unschedule_chance: u32,
    pub swap_chance: u32,
}

impl Default for OptimisationStrategyConfig {
    fn default() -> Self {
        OptimisationStrategyConfig {
            move_chance: 3,
            unschedule_chance: 1,
            swap_chance: 1,
        }
    }
}

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
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub iterations: u32,
    pub number_of_global_searches: u32,
    pub constraint_file_path: String,
    pub initial_temperature: f32,
    pub cooling_factor: f32,
    pub random_seed: u32,
    pub penalties_config: PenaltiesConfig,
    pub optimisation_strategy_config: OptimisationStrategyConfig,
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
            penalties_config: PenaltiesConfig::default(),
            optimisation_strategy_config: OptimisationStrategyConfig::default(),
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
