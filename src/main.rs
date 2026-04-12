use core::panic;

use crate::{
    constraints::{
        constraint_store::{ConstraintStore, load_constraint_store_from_file},
        penalties::print_penalty_report,
    },
    hill_climber::run_hill_climber,
};
use anyhow::Result;
use env_logger;
use log::{error, info};

mod constraints;
mod hill_climber;
mod schedule;

fn main() -> Result<()> {
    env_logger::init();

    info!("Initialising constraint store");
    let mut constraints: ConstraintStore =
        load_constraint_store_from_file("constraints".to_string())
            .expect("Could not load constraints from file. Please ensure the file exists");
    info!("Initalised constraint store");

    info!("Running hill climber algorithm");
    let (schedule, total_incurred_penalty, stats) =
        run_hill_climber(&mut constraints, 100000, 200.0, 0.9999).unwrap_or_else(|error| {
            error!("{}", error);
            panic!();
        });

    info!("Finished hill climber");

    info!("Exporting schedule");
    schedule
        .export_to_csv(String::from("schedule.csv"), &constraints)
        .expect("Could not export to csv");
    info!("Exported schedule");

    constraints.print_schedule_report(&schedule, total_incurred_penalty);
    print_penalty_report(&constraints, &schedule, total_incurred_penalty);

    info!("MOVE COUNT: {}", stats.move_count);
    info!("SCHEDULE COUNT: {}", stats.schedule_count);
    info!(
        "UNSCHEDULE SCHEDULED COUNT: {}",
        stats.unscheduling_scheduled_count
    );
    info!(
        "UNSCHEDULE UNSCHEDULED COUNT: {}",
        stats.unscheduling_unscheduled_count
    );

    Ok(())
}
