use crate::constraints::Constraint;

pub fn calculate_validity_based_penalty(constraint: &Constraint) -> u32 {
    const HIGH_PRIORITY_PENALTY: u32 = 10;
    const LOW_PRIORITY_PENALTY: u32 = 5;

    match constraint.scheduled_slot {
        Some(_) => 0,
        None => match constraint.priority {
            super::ConstraintPriority::High => HIGH_PRIORITY_PENALTY,
            super::ConstraintPriority::Low => LOW_PRIORITY_PENALTY,
        },
    }
}

pub fn calculate_allowed_slots_based_penalty(constraint: &Constraint) -> u32 {
    let allowed_slots = constraint.
        allowed_slots.
        as_ref().
        expect("The allowed slots are not specified, however, the penalty function for allowed slots based penalty was called");

    // NOTE: If it isnt scheduled, the validity based penalty will be applied and we are not going
    // to apply the allowed slots based penalty again
    if let None = constraint.scheduled_slot {
        return 0;
    }

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
    5
}

pub fn calculate_gap_based_penalty(constraint: &Constraint) -> u32 {
    5
}
