pub mod change_types;
pub mod make_small_change;

use crate::{
    constraints::{constraint_store::ConstraintStore, penalties::calculate_penalties},
    hill_climber::make_small_change::SchedulableSlots,
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
    let mut temperature: f32 = 40.0;
    let cooling_factor = 0.995;
    //let mut schedule = generate_naive_schedule(&constraints);
    let mut schedule = Schedule::new();
    let (mut penalties, mut total_penalty) = calculate_penalties(constraints, &schedule);

    let mut best_schedule = schedule.clone();
    let mut best_total_penalty = total_penalty;

    for iteration in 0..iterations {
        println!("Running iteration number {:?}", iteration);
        if let Some(change) = evolve_schedule(constraints, &penalties, &mut schedule) {
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
                    change.revert_change(&mut schedule);
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

    println!(
        "Finished hill climeber optimisation algorithm. Best penalty acheived {}",
        best_total_penalty
    );
    (best_schedule,best_total_penalty)
}

/// Generate a naive schedule as the starting point for the optimisation algorithm
///
/// Iterates through each of the specified constraints and attempts to find an available slot and
/// scheulde them
///
/// # Arguments
/// * constraints - The constraint store containing all constraints
///
/// # Returns
/// * Schedule - The generated naive schedule
pub fn generate_naive_schedule(constraints: &ConstraintStore) -> Schedule {
    let mut schedule = Schedule::new();
    // TODO: We need to consider the constraints in the descending order of their penalties (The
    // iterator should do that anyways)
    for constraint in constraints {
        schedule
            .get_slot_for_constraint(
                constraint.duration,
                &SchedulableSlots {
                    allowed_slots: constraint.allowed_slots.clone(),
                    preferred_slots: constraint.preferred_slots.clone(),
                },
            )
            .and_then(|slot| {
                Some(schedule.schedule_constraint(constraint.id, constraint.duration, &slot))
            });
    }

    schedule
}
