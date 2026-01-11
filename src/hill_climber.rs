use crate::{constraints::Constraint, schedule::Schedule};
use rand::{distr::weighted::WeightedIndex, prelude::*, rng};

pub fn run_hill_climber(constraints: &mut Vec<Constraint>, iterations: u32) {
    let mut best_schedule = Schedule::new();
    let mut best_penalty: u32 = 0;

    let mut schedule = Schedule::new();
    generate_initial_state(constraints, &mut schedule);
    let mut penalty = calculate_penalty(constraints);

    best_schedule = schedule.clone();
    best_penalty = penalty.clone();

    for _ in 0..iterations {
        // TODO: Fix
        make_small_change(constraints.to_vec(), schedule.clone());

        // TODO: Change this to the new constraints
        let new_penalty = calculate_penalty(constraints);
        if new_penalty <= penalty {
            penalty = new_penalty;
            // TODO: Implement
            //schedule = ;

            if new_penalty <= best_penalty {
                best_penalty = penalty;
            }
        }
    }
}

pub fn generate_initial_state(constraints: &mut Vec<Constraint>, schedule: &mut Schedule) {
    // TODO: We need to consider the constraints in the descending order of their priorities

    for constraint in constraints {
        schedule
            .get_slot_for_constraint(constraint)
            .and_then(|slot| Some(schedule.schedule_constraint(constraint, slot)));
    }
}

pub fn calculate_penalty(constraints: &Vec<Constraint>) -> u32 {
    let mut total_penalty = 0;
    for constraint in constraints {
        total_penalty += constraint.calculate_penalty();
    }

    total_penalty
}

// TODO: We may benefit from implementing a constraint store that stores the constraints in the
// order of their penalties in the current schedule
fn make_small_change(mut constraints: Vec<Constraint>, mut schedule: Schedule) {
    let mut constraint = constraints
        .choose_mut(&mut rand::rng())
        .expect("Could not sample constraint from constraints in make small change. Ensure atleast one constraint is specified.");

    match constraint.is_scheduled() {
        true => {
            attempt_to_reduce_constraint_penalty(&mut constraint, &mut schedule);
        }
        false => {
            attempt_to_schedule_constraint(&mut constraint, &mut schedule);
        }
    }
}

fn attempt_to_reduce_constraint_penalty(constraint: &mut Constraint, schedule: &mut Schedule) {
    let option_weightings = vec![0.5, 0.5];
    let distribution = WeightedIndex::new(&option_weightings)
        .expect("Invalid weights for constraint option weighintgs");
    let selected_option = distribution.sample(&mut rng());

    match selected_option {
        0 => {attempt_to_move_constraint(constraint, schedule)},
        _ => {
            // TODO: Cannot do without reworking
            attempt_to_swap_constraint(constraint, schedule)
        }
    }
}

fn attempt_to_schedule_constraint(constraint: &mut Constraint, schedule: &mut Schedule) {
    if let Some(slot) = schedule.get_slot_for_constraint(constraint) {
        schedule.schedule_constraint(constraint, slot);
    } else {
        // TODO: Stochastically choose an item to swap out and swap in the new constraint
    }
}
