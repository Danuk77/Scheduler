use crate::{constraints::Constraint, schedule::Schedule};

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
