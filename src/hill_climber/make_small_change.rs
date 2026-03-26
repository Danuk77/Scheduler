use crate::{
    constraints::constraint_store::ConstraintStore,
    hill_climber::change_types::ChangeType,
    schedule::{Schedule, Slot},
};
use std::collections::HashMap;

/// Structure used for passing allowed slots and preferred slots as one
pub struct SchedulableSlots {
    pub allowed_slots: Option<Vec<Slot>>,
    pub preferred_slots: Option<Vec<Slot>>,
}

///  Runs a single iteration of the hill climbing optimisation algorithm
///
///  # Arguments
///  * `constraints` - The constraints store containing all constraints
///  * `incurred_penalties` - The penalties incurred by the constraints under the specified schedule
///  * `Schedule` - The current state of the schedule
///
///  # Returns
///  `ChangeType` - The type of change made if the state of the schedule is changed
///  `None` - If no state change was done on this iteration
pub fn evolve_schedule(
    constraints: &mut ConstraintStore,
    incurred_penalties: &HashMap<u32, u32>,
    schedule: &mut Schedule,
) -> Option<Vec<ChangeType>> {
    let constraint = constraints.get_constraint_for_optimisation(incurred_penalties)?;

    println!("Constraint {:?} choosen for optimisation", constraint.name);
    let constraint_id = constraint.id;
    let constraint_duration = constraint.duration;
    let schedulabe_slots_for_constraint = SchedulableSlots {
        allowed_slots: constraint.allowed_slots.clone(),
        preferred_slots: constraint.preferred_slots.clone(),
    };

    if schedule.is_constraint_scheduled(constraint_id) {
        return handle_scheduled_constraint(
            constraint_id,
            constraint_duration,
            schedulabe_slots_for_constraint,
            schedule,
            constraints,
        );
    } else {
        return handle_unscheduled_constraint(
            constraint_id,
            constraint_duration,
            schedulabe_slots_for_constraint,
            schedule,
            constraints,
        );
    }
}

/// Function called when trying to optimise a constraint that is already scheuled
///
/// # Arguments
/// * constraint - The constraint to optimise
/// * schedule - The current state of the schedule
///
/// # Returns
/// * Vec<ChangeType> - A vector containing all the changes performed (in order)
/// * None - If no optimisation was performed
fn handle_scheduled_constraint(
    constraint_id: u32,
    constraint_duration: u8,
    schedulable_slots: SchedulableSlots,
    schedule: &mut Schedule,
    constraint_store: &ConstraintStore,
) -> Option<Vec<ChangeType>> {
    let mut changes_made: Vec<ChangeType> = Vec::new();
    let alternative_slot =
        schedule.get_slot_for_constraint(constraint_duration, &schedulable_slots);

    match alternative_slot {
        Some(slot) => {
            let previous_slot = schedule
                .unschedule_constraint(constraint_id, constraint_duration)
                .expect("Unexpected logic error");
            changes_made.push(ChangeType::Unscheduled(
                constraint_id,
                constraint_duration,
                previous_slot,
            ));

            schedule.schedule_constraint(constraint_id, constraint_duration, &slot);
            changes_made.push(ChangeType::Scheduled(constraint_id, constraint_duration));

            return Some(changes_made);
        }
        None => {
            if let Some(swappable_constraint) = constraint_store
                .find_swappable_scheduled_constraint(constraint_id, constraint_duration, schedule)
            {
                let freed_slot = schedule
                    .unschedule_constraint(swappable_constraint.id, swappable_constraint.duration)
                    .expect("Unexpected error ocurred whilst unscheduling constraint");
                changes_made.push(ChangeType::Unscheduled(
                    swappable_constraint.id,
                    swappable_constraint.duration,
                    freed_slot.clone(),
                ));

                schedule.schedule_constraint(constraint_id, constraint_duration, &freed_slot);
                changes_made.push(ChangeType::Scheduled(constraint_id, constraint_duration));
                return Some(changes_made);
            }
            println!("Cannot optimise constraint (scheduled)");
            None
        }
    }
}

/// Function called when trying to optimise a constraint that is not yet scheduled
///
/// # Arguments
/// * constraint_id - The id of the constraint to be optimised
/// * constraint_duration - The duration of the constraint to be optimised
/// * allowed_slots_for_constraint - The slots the constraint is allowed to take
/// * schedule - The current state of the schedule
/// * constraint_store - The store containing all constraints
///
/// # Returns
/// * Vec<ChangeType> - A vector containing all the changes performed (in order)
/// * None - If no optimisation was performed
fn handle_unscheduled_constraint(
    constraint_id: u32,
    constraint_duration: u8,
    schedulable_slots: SchedulableSlots,
    schedule: &mut Schedule,
    constraint_store: &ConstraintStore,
) -> Option<Vec<ChangeType>> {
    let mut changes_made: Vec<ChangeType> = Vec::new();

    if let Some(slot) = schedule.get_slot_for_constraint(constraint_duration, &schedulable_slots) {
        schedule.schedule_constraint(constraint_id, constraint_duration, &slot);
        return vec![ChangeType::Scheduled(constraint_id, constraint_duration)].into();
    }

    if let Some(swappable_constraint) = constraint_store.find_swappable_scheduled_constraint(
        constraint_id,
        constraint_duration,
        schedule,
    ) {
        let freed_slot = schedule
            .unschedule_constraint(swappable_constraint.id, swappable_constraint.duration)
            .expect("Unexpected error ocurred whilst unscheduling constraint");
        changes_made.push(ChangeType::Unscheduled(
            swappable_constraint.id,
            swappable_constraint.duration,
            freed_slot.clone(),
        ));

        schedule.schedule_constraint(constraint_id, constraint_duration, &freed_slot);
        changes_made.push(ChangeType::Scheduled(constraint_id, constraint_duration));
        return Some(changes_made);
    }

    // TODO:
    // 1. Randomly find a compatible starting slot for the constraint
    // 2. Unschedule all constraints that are scheduled under that slot
    // 3. Schedule in the new slot

    println!("Cannot optimise constraint (not scheduled)");
    None
}
