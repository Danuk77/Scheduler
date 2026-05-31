use core::panic;

use crate::{
    config::Config,
    constraints::{
        constraint_store::{ConstraintStore, load_constraint_store_from_file},
        penalties::print_penalty_report,
    },
    global_search::run_global_search,
};
use anyhow::Result;
use env_logger;
use log::error;

mod config;
mod constraints;
mod global_search;
mod hill_climber;
mod random;
mod schedule;
mod stats;

fn main() -> Result<()> {
    env_logger::init();
    let config = Config::new()?;

    let mut constraints: ConstraintStore =
        load_constraint_store_from_file(config.constraint_file_path)
            .expect("Could not load constraints from file. Please ensure the file exists");

    let (schedule, total_incurred_penalty, stats) = run_global_search(
        &mut constraints,
        config.iterations,
        config.initial_temperature,
        config.cooling_factor,
        config.number_of_global_searches,
        config.random_seed,
    )
    .unwrap_or_else(|error| {
        error!("{}", error);
        panic!();
    });

    schedule
        .export_to_csv(config.output_path, &constraints)
        .expect("Could not export to csv");

    stats.generate_optimisation_report();
    constraints.print_schedule_report(&schedule, total_incurred_penalty);
    print_penalty_report(&constraints, &schedule, total_incurred_penalty);

    Ok(())
}
