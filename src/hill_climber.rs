pub mod change_types;
pub mod make_small_change;

use crate::{
    constraints::{constraint_store::ConstraintStore, penalties::calculate_penalties},
    schedule::Schedule,
};
use make_small_change::evolve_schedule;
use rand::random;

/// Runs hill climbing optimisation algorithm to generate a schedule to satisfied specified
/// constraints
///
/// # Arguments
/// * constraints - The constraints store containing all the constraints to satisfy
/// * iterations - The number of iterations to run the optimisation algorithm for
///
/// # Returns
/// * Schedule - The output of the optimisation algorithm
pub fn run_hill_climber(constraints: &mut ConstraintStore, iterations: u32) -> (Schedule, u32) {
    let mut temperature: f32 = 200.0;
    let cooling_factor = 0.9999;
    let mut schedule = Schedule::new();
    let (mut penalties, mut total_penalty) = calculate_penalties(constraints, &schedule);

    let mut best_schedule = schedule.clone();
    let mut best_total_penalty = total_penalty;

    for iteration in 0..iterations {
        println!("Running iteration number {:?}", iteration);
        if let Some(changes) = evolve_schedule(constraints, &penalties, &mut schedule) {
            let (new_penalties, new_total_penalty) = calculate_penalties(constraints, &schedule);
            println!("Evaluated penalty. Penalty: {:?}", new_total_penalty);

            if new_total_penalty <= total_penalty {
                total_penalty = new_total_penalty;
                penalties = new_penalties;

                if total_penalty <= best_total_penalty {
                    best_total_penalty = total_penalty;
                    best_schedule = schedule.clone();
                }
            } else {
                println!("New penalty is worse than existing best penalty");
                let delta_penalty: f32 = new_total_penalty as f32 - total_penalty as f32;
                if (-delta_penalty / temperature).exp() > random() {
                    println!("Accepting worse schedule");
                    total_penalty = new_total_penalty;
                    penalties = new_penalties;
                } else {
                    println!("Reverting");
                    changes
                        .iter()
                        .rev()
                        .for_each(|change| change.revert_change(&mut schedule));
                }
            }
            temperature *= cooling_factor;
        } else {
            println!(
                "Did not find optimisation schedule at iteration {:?}",
                iteration
            );
        }
    }

    (best_schedule, best_total_penalty)
}
