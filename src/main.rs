use crate::{
    constraints::constraint_store::{ConstraintStore, load_constraint_store_from_file},
    hill_climber::run_hill_climber,
};

mod constraints;
mod hill_climber;
mod schedule;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Initialising constraint store");
    let mut constraints: ConstraintStore =
        load_constraint_store_from_file("constraints".to_string())
            .expect("Could not load constraints from file. Please ensure the file exists");

    println!("Running hill climber algorithm");
    let (schedule, total_incurred_penalty) = run_hill_climber(&mut constraints, 1000000);
    schedule
        .export_to_csv(String::from("schedule.csv"), &constraints)
        .expect("Could not export to csv");

    println!("Finished hill climber");

    constraints.print_schedule_report(&schedule, total_incurred_penalty);
    Ok(())
}
