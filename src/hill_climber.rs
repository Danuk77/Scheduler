pub mod change_types;
pub mod make_small_change;

use crate::{constraints::constraint_store::ConstraintStore, schedule::Schedule};
use make_small_change::make_small_change;

pub fn run_hill_climber(constraints: &mut ConstraintStore, iterations: u32) -> Schedule {
    let mut schedule = Schedule::new();

    generate_naive_schedule(&constraints, &mut schedule);
    let mut penalty = constraints.calculate_penalties(&schedule);

    let mut best_schedule = schedule.clone();
    let mut best_penalty = penalty.clone();

    for _ in 0..iterations {
        // TODO: Return enum of what change was made from this function
        let change = make_small_change(constraints, &mut schedule);
        let new_penalty = constraints.calculate_penalties(&schedule);

        if new_penalty <= penalty {
            penalty = new_penalty;
            if penalty <= best_penalty {
                best_penalty = penalty;
                best_schedule = schedule.clone();
            }
        } else {
            // TODO: revert the change back into the old schedule
        }
    }

    best_schedule
}

pub fn generate_naive_schedule(constraints: &ConstraintStore, schedule: &mut Schedule) {
    // TODO: We need to consider the constraints in the descending order of their penalties (The
    // iterator should do that anyways)
    for constraint in constraints {
        schedule
            .get_slot_for_constraint(constraint)
            .and_then(|slot| {
                Some(schedule.schedule_constraint(constraint.id, constraint.duration, &slot))
            });
    }
}
