use crate::{
    constraints::constraint_store::ConstraintStore,
    hill_climber::change_types::ChangeType,
    schedule::{Schedule, Slot},
};

///  Runs a single iteration of the hill climbing optimisation algorithm
///
///  # Arguments
///  * constraints - The constraints store containing all constraints
///  * Schedule - The current state of the schedule
///
///  # Returns
///  ChangeType - The type of change made if the state of the schedule is changed
///  None - If no state change was done on this iteration
pub fn evolve_schedule(
    constraints: &mut ConstraintStore,
    schedule: &mut Schedule,
) -> Option<ChangeType> {
    let constraint = constraints.get_constraint_for_adjustment()?;
    let constraint_id = constraint.id;
    let constraint_duration = constraint.duration;
    let allowed_slots = constraint.allowed_slots.clone();

    if schedule.is_constraint_scheduled(constraint_id) {
        return handle_scheduled_constraint(
            constraint_id,
            constraint_duration,
            allowed_slots,
            schedule,
        );
    } else {
        return handle_unscheduled_constraint(
            constraint_id,
            constraint_duration,
            allowed_slots,
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
/// * ChangeType - The type of optimisation performed
/// * None - If no optimisation was performed
fn handle_scheduled_constraint(
    constraint_id: u32,
    constraint_duration: u8,
    allowed_slots: Option<Vec<Slot>>,
    schedule: &mut Schedule,
) -> Option<ChangeType> {
    let alternative_slot = find_alternative_slot(constraint_duration, allowed_slots, schedule);

    match alternative_slot {
        Some(slot) => {
            let previous_slot = schedule
                .unschedule_constraint(constraint_id, constraint_duration)
                .expect("Unexpected logic error");
            schedule.schedule_constraint(constraint_id, constraint_duration, &slot);
            return Some(ChangeType::Move(previous_slot, slot.clone()));
        }
        None => {
            println!("Cannot optimise constraint (scheuled)");
            None
        }
    }
}

/// Finds an empty slot for a constraint from the schedule
///
/// # Arguments
/// * constraint_duration - The duration of the constraint to find a slot for
/// * allowed_slots - The slots the constraint is allowed to be scheduled in
/// * schedule - The current state of the schedule
///
/// # Returns
/// Slot - If slot is found
/// None - If no free slot exists for constraint
fn find_alternative_slot(
    constraint_duration: u8,
    allowed_slots: Option<Vec<Slot>>,
    schedule: &Schedule,
) -> Option<Slot> {
    if let Some(slot) = allowed_slots {
        return slot
            .iter()
            .find(|slot| schedule.is_duration_free(&slot, constraint_duration))
            .cloned();
    }

    return schedule.get_slot_for_constraint(constraint_duration, &allowed_slots);
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
/// * ChangeType - The type of optimisation performed
/// * None - If no optimisation was performed
fn handle_unscheduled_constraint(
    constraint_id: u32,
    constraint_duration: u8,
    allowed_slots_for_constraint: Option<Vec<Slot>>,
    schedule: &mut Schedule,
    constraint_store: &ConstraintStore,
) -> Option<ChangeType> {
    if let Some(slot) =
        schedule.get_slot_for_constraint(constraint_duration, &allowed_slots_for_constraint)
    {
        schedule.schedule_constraint(constraint_id, constraint_duration, &slot);
        return Some(ChangeType::Scheduled(constraint_id));
    }

    if let Some(swappable_constraint) = constraint_store.find_swappable_scheduled_constraint(
        constraint_id,
        constraint_duration,
        schedule,
    ) {
        let freed_slot = schedule
            .unschedule_constraint(swappable_constraint.id, swappable_constraint.duration)
            .expect("Unexpected error ocurred whilst unscheduling constraint");
        schedule.schedule_constraint(constraint_id, constraint_duration, &freed_slot);
        return Some(ChangeType::Subtituted(
            constraint_id,
            swappable_constraint.id,
        ));
    }

    println!("Cannot optimise constraint (not scheduled)");
    None
}
