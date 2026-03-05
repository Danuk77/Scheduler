use std::collections::HashMap;

use crate::{
    constraints::{Constraint, constraint_store::ConstraintStore},
    schedule::Schedule,
};

/// Calculates the penalties for a set of constraints under a certain schedule
///
/// # Arguments
/// * `constraints` - The constraints to evaluate the penalties for
/// * `schedule` - The schedule to evaluate the constraints for penalties under
///
/// # Returns
/// (
///     HashMap<u32, u32> - A hashmap with keys being constraint id and the value being the
///     calculated penalty,
///     u32 - The total penalty incurred
/// )
pub fn calculate_penalties(
    constraints: &ConstraintStore,
    schedule: &Schedule,
) -> (HashMap<u32, u32>, u32) {
    let mut penalties: HashMap<u32, u32> = HashMap::new();
    let mut total_penalty = 0;

    for constraint in constraints.into_iter() {
        let constraint_penalty = constraint.calculate_penalty(schedule);
        penalties.insert(constraint.id, constraint_penalty);
        total_penalty += constraint_penalty;
    }

    (penalties, total_penalty)
}

/// Calculates the penalty applied for a constraint depending on whether it is scheduled or not
///
/// # Arguments
/// * `constraint` - The constraint to calculate the penalty for
/// * `schedule` - The schedule the constraint is scheduled in
///
/// # Returns
/// `u32` - The calculated penalty for constraint
pub fn calculate_presence_based_penalty(constraint: &Constraint, schedule: &Schedule) -> u32 {
    if !schedule.is_constraint_scheduled(constraint.id) {
        match constraint.priority {
            super::ConstraintPriority::High => return 10,
            super::ConstraintPriority::Low => return 5,
        }
    }

    0
}

/// Calculated the penalty based on whether the constraint is scheduled in a slot that it is
/// allowed to be scheduled or not
/// NOTE: An allowed slot is one that is mandatory for the constraint to be scheduled in.
/// If not scheduled in an allowed slot, a high penalty will be applied to the constraint
///
/// # Arguments
/// * `constraint` - The constraint to calculate the penalty for
/// * `schedule` - The schedule which the penalty should be evaluated under
///
/// # Returns
/// * `u32` - The calculated penalty
pub fn calculate_allowed_slots_based_penalty(constraint: &Constraint, schedule: &Schedule) -> u32 {
    // NOTE: If it isnt scheduled, the validity based penalty will be applied and we are not going
    // to apply the allowed slots based penalty again
    let Some(scheduled_slot) = schedule.get_scheduled_slot_for_constraint(constraint.id) else {
        return 0;
    };

    let allowed_slots = constraint.
        allowed_slots.
        as_ref().
        expect("The allowed slots are not specified, however, the penalty function for allowed slots based penalty was called");

    if !allowed_slots.contains(scheduled_slot) {
        match constraint.priority {
            super::ConstraintPriority::High => return 30,
            super::ConstraintPriority::Low => return 20,
        }
    }

    0
}

/// Calculated the penalty based on whether the constraint is scheduled in a slot that it is
/// preferred to be scheduled in or not.
/// NOTE: A preferred slot is one that is not mandatory for the constraint to be scheduled in,
/// however would incurr less penalty if scheduled in
///
/// # Arguments
/// * `constraint` - The constraint to calculate the penalty for
/// * `schedule` - The schedule which the penalty should be evaluated under
///
/// # Returns
/// * `u32` - The calculated penalty
pub fn calculate_preferred_slots_based_penalty(
    constraint: &Constraint,
    schedule: &Schedule,
) -> u32 {
    // NOTE: If it isnt scheduled, the validity based penalty will be applied and we are not going
    // to apply the allowed slots based penalty again
    let Some(scheduled_slot) = schedule.get_scheduled_slot_for_constraint(constraint.id) else {
        return 0;
    };

    let preferred_slots = constraint.
        preferred_slots.
        as_ref().
        expect("The preferred slots are not specified, however, the penalty function for preferred slots based penalty was called");

    if !preferred_slots.contains(scheduled_slot) {
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
