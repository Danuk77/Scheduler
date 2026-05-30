use core::panic;

use crate::{
    constraints::{
        constraint_store::{ConstraintStore, load_constraint_store_from_file},
        penalties::print_penalty_report,
    },
    global_search::run_global_search,
};
use anyhow::Result;
use env_logger;
use log::{error, info};

mod constraints;
mod global_search;
mod hill_climber;
mod random;
mod schedule;
mod stats;

fn main() -> Result<()> {
    env_logger::init();

    info!("Initialising constraint store");
    let mut constraints: ConstraintStore =
        load_constraint_store_from_file("constraints".to_string())
            .expect("Could not load constraints from file. Please ensure the file exists");
    info!("Initalised constraint store");

    let (schedule, total_incurred_penalty, stats) =
        run_global_search(&mut constraints, 100000, 200.0, 0.9999, 10, 123).unwrap_or_else(
            |error| {
                error!("{}", error);
                panic!();
            },
        );

    info!("Exporting schedule");
    schedule
        .export_to_csv(String::from("schedule.csv"), &constraints)
        .expect("Could not export to csv");
    info!("Exported schedule");

    stats.generate_optimisation_report();
    constraints.print_schedule_report(&schedule, total_incurred_penalty);
    print_penalty_report(&constraints, &schedule, total_incurred_penalty);

    Ok(())
}
