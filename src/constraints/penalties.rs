use std::collections::HashMap;

use log::info;

use crate::{
    constraints::{Constraint, constraint_store::ConstraintStore, penalty::Penalty},
    schedule::{Schedule, Slot},
};

/// Calculates the penalties for a set of constraints under a certain schedule
///
/// This function is more expensive as it will store each of the penalties incurred per penalty
/// type basis. This function is mainly used for debugging and reporting purposes and not for
/// evaluating the penalty in each of the iterations of the optimisation algorithm
///
/// # Arguments
/// * `constraints` - The constraints to evaluate the penalties for
/// * `schedule` - The schedule to evaluate the constraints for penalties under
///
/// # Returns
/// (
///     HashMap<
///         u32 - The constraint id,
///         (
///             Penalty - The incurred penalty type,
///             u32 - The value of the incurred penalty under that type
///         )
///     >
pub fn calculate_detailed_penalties(
    constraints: &ConstraintStore,
    schedule: &Schedule,
) -> HashMap<u32, Vec<(Penalty, u32)>> {
    let mut penalties: HashMap<u32, Vec<(Penalty, u32)>> = HashMap::new();

    for constraint in constraints.into_iter() {
        let constraint_penalty = constraint.calculate_detailed_penalty(schedule, constraints);
        penalties.insert(constraint.id, constraint_penalty);
    }

    penalties
}

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
        let constraint_penalty = constraint.calculate_penalty(schedule, constraints);
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

/// Calculates the penalty incurred if the minimum gap requirement between
/// constraints of the same type is violated.
///
///
/// # Arguments
/// * `constraint` - The current constraint being evaluated.
/// * `schedule` - The current state of the schedule, used to check where other constraints are placed.
/// * `constraint_store` - The repository of all constraints, used to find others of the same type.
///
/// # Returns
/// * `3` - If a gap violation is found with any other scheduled constraint of the same type.
/// * `0` - If no violations are found or no other constraints of the same type are scheduled.
///
/// # Panics
/// * Panics if the `constraint` does not have a defined `gap` value.
/// * Panics if a matching constraint is marked as scheduled but its slot cannot be retrieved.
pub fn calculate_gap_based_penalty(
    constraint: &Constraint,
    schedule: &Schedule,
    constraint_store: &ConstraintStore,
) -> u32 {
    // NOTE: If the constraint is not scheduled then we do not calculate the gap based penalty
    if !schedule.is_constraint_scheduled(constraint.id) {
        return 0;
    }

    let same_type_constraints = constraint_store.get_constraint_ids_of_type(&constraint.name);

    for constraint_id in same_type_constraints {
        if !schedule.is_constraint_scheduled(constraint_id) || constraint_id == constraint.id {
            continue;
        }

        let gap = calculate_gap_between_slots(
            schedule
                .get_scheduled_slot_for_constraint(constraint_id)
                .unwrap(),
            schedule
                .get_scheduled_slot_for_constraint(constraint.id)
                .unwrap(),
        );

        if gap < constraint.gap.expect("Unexpected call calculating gap based penalty when no gap was specified for constraint") {
            return 3;
        }
    }

    0
}

/// Calculates the number of slots between the specified two slots
///
/// # Arguments
/// * `slot_one` - The first slot
/// * `slot_two` - The second slot
///
/// # Returns
/// * `u16` - The total number of slots between the two slots
fn calculate_gap_between_slots(slot_one: &Slot, slot_two: &Slot) -> u16 {
    let day_gap = slot_one.day as i16 - slot_two.day as i16;
    let window_gap = slot_one.window as i16 - slot_two.window as i16;

    ((day_gap * 48) + window_gap).unsigned_abs()
}

/// Prints a penalty report that consists of the total penalty incurred followed by
/// the penalties for each of the individual constraints (on a penalty type basis).
///
/// NOTE: The formatting is AI generated. Didnt want to spend time on that
///
/// # Arguments
/// * `penalties` - Hashmap containing the penalties incurred
/// * `constraints` - The constraint store containing all penalties
/// * `total_incurred_penalty` - The total incurred penalty
pub fn print_penalty_report(
    constraints: &ConstraintStore,
    schedule: &Schedule,
    total_incurred_penalty: u32,
) {
    let penalties = calculate_detailed_penalties(constraints, schedule);

    // Header - Minimalist and wide-screen friendly
    info!("=== PENALTY REPORT SUMMARY ===");
    info!("TOTAL INCURRED PENALTY: {}", total_incurred_penalty);
    info!("---");

    let mut collected: Vec<_> = penalties.iter().collect();

    collected.sort_by_key(|p| std::cmp::Reverse(p.1.iter().map(|(_, v)| v).sum::<u32>()));

    for (id, instances) in collected {
        let constraint_name = constraints
            .get_constraint(*id)
            .map(|c| c.name.as_str())
            .unwrap_or("Unknown Constraint");

        let total_for_group: u32 = instances.iter().map(|(_, v)| v).sum::<u32>();

        info!(
            "[{}:{}] Total: {}",
            id,
            constraint_name.to_uppercase(),
            total_for_group
        );

        for (reason, value) in instances {
            // This format string is flexible; no vertical bars to misalign
            info!("  └─ {}: {}", reason, value);
        }

        info!("");
    }

    info!("==============================");
}
