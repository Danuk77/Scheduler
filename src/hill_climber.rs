pub mod change_types;
pub mod make_small_change;

use std::error::Error;

use crate::{
    constraints::{constraint_store::ConstraintStore, penalties::calculate_penalties},
    schedule::Schedule,
};
use log::debug;
use make_small_change::evolve_schedule;
use rand::random;

/// Runs hill climbing optimisation algorithm to generate a schedule to satisfy specified
/// constraints
///
/// # Arguments
/// * `constraints` - The constraints store containing all the constraints to satisfy
/// * `iterations` - The number of iterations to run the optimisation algorithm for
/// * `temperature` - The initial temperature to run the hill climber with
/// * `cooling_factor` - The cooling factor for the temperature
///
/// # Returns
/// * Schedule - The output of the optimisation algorithm
pub fn run_hill_climber(
    constraints: &mut ConstraintStore,
    iterations: u32,
    initial_temperature: f32,
    cooling_factor: f32,
) -> Result<(Schedule, u32), Box<dyn Error>> {
    let mut schedule = Schedule::new();
    let (mut penalties, mut total_penalty) = calculate_penalties(constraints, &schedule);
    let mut temperature = initial_temperature;
    let mut stagnant_counter = 0;

    let mut best_schedule = schedule.clone();
    let mut best_total_penalty = total_penalty;

    for iteration in 0..iterations {
        debug!("Running iteration number {:?}", iteration);
        temperature *= cooling_factor;

        let Some(changes) = evolve_schedule(constraints, &penalties, &mut schedule)? else {
            debug!(
                "Did not find optimisation schedule at iteration {:?}",
                iteration
            );
            stagnant_counter += 1;
            continue;
        };

        let (new_penalties, new_total_penalty) = calculate_penalties(constraints, &schedule);
        debug!("Evaluated penalty. Penalty: {:?}", new_total_penalty);

        if !_should_accept_schedule(new_total_penalty, total_penalty, temperature) {
            debug!("Evolution not accepted. Reverting");
            stagnant_counter += 1;
            changes
                .iter()
                .rev()
                .for_each(|change| change.revert_change(&mut schedule));

            if stagnant_counter >= 500 {
                stagnant_counter = 0;
                temperature = initial_temperature;
            }
            continue;
        };

        total_penalty = new_total_penalty;
        penalties = new_penalties;

        if total_penalty <= best_total_penalty {
            best_total_penalty = total_penalty;
            best_schedule = schedule.clone();
        }
    }

    Ok((best_schedule, best_total_penalty))
}

/// Evaluated whether an evolution in the schedule should be accepted or not based on the realised
/// penalties
///
/// # Arguments
/// * `new_total_penalty` - The penalties realised for the new schedule
/// * `existing_total_penalty` - The penalties realised for the schedule before the evolution
/// * `temperature` - The current temperature in the annealing algorithm
///
/// # Returns
/// * `true` - If the evolution should be accepted
/// * `false` - Otherwise
fn _should_accept_schedule(
    new_total_penalty: u32,
    existing_total_penalty: u32,
    temperature: f32,
) -> bool {
    let delta_penalty: f32 = new_total_penalty as f32 - existing_total_penalty as f32;

    if delta_penalty <= 0.0 {
        return true;
    }

    // NOTE: This is to prevent a divide by zero error
    if temperature < 0.00001 {
        return false;
    };

    if (-delta_penalty / temperature).exp() > random() {
        return true;
    }

    false
}
