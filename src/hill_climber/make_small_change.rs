use log::debug;

use crate::{
    constraints::constraint_store::ConstraintStore,
    hill_climber::change_types::ChangeType,
    schedule::{Schedule, Slot},
};
use std::{collections::HashMap, error::Error};

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
) -> Result<Option<Vec<ChangeType>>, Box<dyn Error>> {
    let constraint = constraints.get_constraint_for_optimisation(incurred_penalties)?;

    debug!("Constraint {:?} choosen for optimisation", constraint.name);
    let constraint_id = constraint.id;
    let constraint_duration = constraint.duration;
    let schedulabe_slots_for_constraint = SchedulableSlots {
        allowed_slots: constraint.allowed_slots.clone(),
        preferred_slots: constraint.preferred_slots.clone(),
    };

    if schedule.is_constraint_scheduled(constraint_id) {
        return Ok(handle_scheduled_constraint(
            constraint_id,
            constraint_duration,
            schedulabe_slots_for_constraint,
            schedule,
        ));
    } else {
        return Ok(handle_unscheduled_constraint(
            constraint_id,
            constraint_duration,
            schedulabe_slots_for_constraint,
            schedule,
        ));
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
) -> Option<Vec<ChangeType>> {
    let mut changes_made: Vec<ChangeType> = Vec::new();
    let alternative_slot =
        schedule.get_free_slot_for_constraint(constraint_duration, &schedulable_slots);

    match alternative_slot {
        Some(slot) => {
            let previous_slot = schedule
                .unschedule_constraint(constraint_id)
                .expect("Unexpected logic error");
            changes_made.push(ChangeType::Unscheduled(
                constraint_id,
                constraint_duration,
                previous_slot,
            ));

            schedule.schedule_constraint(constraint_id, constraint_duration, &slot);
            changes_made.push(ChangeType::Scheduled(constraint_id));

            return Some(changes_made);
        }
        None => {
            let freed_slot = schedule.unschedule_constraint(constraint_id).unwrap();
            let mut changes_made: Vec<ChangeType> = vec![ChangeType::Unscheduled(
                constraint_id,
                constraint_duration,
                freed_slot.clone(),
            )];

            let slot = schedule.choose_slot_for_constraint(constraint_duration);
            let unscheduled_constraints = schedule
                .unschedule_constraints_under_duration_from_slot(&slot, constraint_duration);

            unscheduled_constraints
                .iter()
                .for_each(|c| changes_made.push(ChangeType::Unscheduled(c.0, c.1, c.2.clone())));

            schedule.schedule_constraint(constraint_id, constraint_duration, &slot);
            changes_made.push(ChangeType::Scheduled(constraint_id));
            Some(changes_made)
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
) -> Option<Vec<ChangeType>> {
    match schedule.get_free_slot_for_constraint(constraint_duration, &schedulable_slots) {
        Some(slot) => {
            schedule.schedule_constraint(constraint_id, constraint_duration, &slot);
            Some(vec![ChangeType::Scheduled(constraint_id)])
        }
        None => {
            let slot = schedule.choose_slot_for_constraint(constraint_duration);
            let unscheduled_constraints = schedule
                .unschedule_constraints_under_duration_from_slot(&slot, constraint_duration);
            schedule.schedule_constraint(constraint_id, constraint_duration, &slot);
            let mut changes_made: Vec<ChangeType> = unscheduled_constraints
                .iter()
                .map(|c| ChangeType::Unscheduled(c.0, c.1, c.2.clone()))
                .collect();
            changes_made.push(ChangeType::Scheduled(constraint_id));
            Some(changes_made)
        }
    }
}
