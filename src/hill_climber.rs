pub mod change_types;
pub mod make_small_change;

use crate::{
    constraints::{constraint_store::ConstraintStore, penalties::calculate_penalties},
    schedule::Schedule,
};
use make_small_change::evolve_schedule;

/// Runs hill climbing optimisation algorithm to generate a schedule to satisfied specified
/// constraints
///
/// # Arguments
/// * constraints - The constraints store containing all the constraints to satisfy
/// * iterations - The number of iterations to run the optimisation algorithm for
///
/// # Returns
/// * Schedule - The output of the optimisation algorithm
pub fn run_hill_climber(constraints: &mut ConstraintStore, iterations: u32) -> Schedule {
    let mut schedule = generate_naive_schedule(&constraints);
    let (mut penalties, mut total_penalty) = calculate_penalties(constraints, &schedule);

    let mut best_schedule = schedule.clone();
    let mut best_total_penalty = total_penalty;

    for _ in 0..iterations {
        let change = evolve_schedule(constraints, &penalties, &mut schedule);
        let (new_penalties, new_total_penalty) = calculate_penalties(constraints, &schedule);

        if new_total_penalty <= total_penalty {
            total_penalty = new_total_penalty;
            penalties = new_penalties;
            if total_penalty <= best_total_penalty {
                best_total_penalty = total_penalty;
                best_schedule = schedule.clone();
            }
        } else {
            // TODO: revert the change back into the old schedule
        }
    }

    best_schedule
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
            .get_slot_for_constraint(constraint.duration, &constraint.allowed_slots)
            .and_then(|slot| {
                Some(schedule.schedule_constraint(constraint.id, constraint.duration, &slot))
            });
    }

    schedule
}
