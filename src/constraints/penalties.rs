use crate::constraints::Constraint;

pub fn calculate_validity_based_penalty(constraint: &Constraint) -> u32 {
    if is_constraint_scheduled(constraint) {
        match constraint.priority {
            super::ConstraintPriority::High => return 10,
            super::ConstraintPriority::Low => return 5,
        }
    }

    0
}

fn is_constraint_scheduled(constraint: &Constraint) -> bool {
    if let None = constraint.scheduled_slot {
        return false;
    }

    true
}

pub fn calculate_allowed_slots_based_penalty(constraint: &Constraint) -> u32 {
    // NOTE: If it isnt scheduled, the validity based penalty will be applied and we are not going
    // to apply the allowed slots based penalty again
    if !is_constraint_scheduled(constraint) {
        return 0;
    }

    let allowed_slots = constraint.
        allowed_slots.
        as_ref().
        expect("The allowed slots are not specified, however, the penalty function for allowed slots based penalty was called");

    // NOTE: The reason that we return such a high value if scheduled in a non allowed slot is
    // because we want the constraint to be never scheduled in a non allowed slot
    if !allowed_slots.contains(&constraint.scheduled_slot.unwrap()) {
        match constraint.priority {
            super::ConstraintPriority::High => return 30,
            super::ConstraintPriority::Low => return 20,
        }
    }

    0
}

pub fn calculate_preferred_slots_based_penalty(constraint: &Constraint) -> u32 {
    // NOTE: If it isnt scheduled, the validity based penalty will be applied and we are not going
    // to apply the allowed slots based penalty again
    if !is_constraint_scheduled(constraint) {
        return 0;
    }

    let preferred_slots = constraint.
        preferred_slots.
        as_ref().
        expect("The preferred slots are not specified, however, the penalty function for preferred slots based penalty was called");

    if !preferred_slots.contains(&constraint.scheduled_slot.unwrap()) {
        match constraint.priority {
            super::ConstraintPriority::High => return 3,
            super::ConstraintPriority::Low => return 2,
        }
    }

    0
}

// TODO: Implement
pub fn calculate_gap_based_penalty(constraint: &Constraint) -> u32 {
    0
}
