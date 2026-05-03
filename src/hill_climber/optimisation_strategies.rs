use super::make_small_change::SchedulableSlots;
use crate::{
    constraints::constraint_store::ConstraintStore,
    hill_climber::change_types::ChangeType,
    schedule::{Schedule, Slot},
};

/// Enum defining the different optimisation strategies
pub enum OptimisationStrategy {
    MOVE,
    SWAP,
    UNSCHEDULE,
    SCHEDULE,
}

/// Executes strategy where a constraint is moved from its current scheduled slot to another
/// slot
///
/// # Arguments
/// * `schedule` - The scheduled where the constraint is scheduled in
/// * `constraint_id` - The id of the constraint to move
/// * `constraint_duration` - The duration of the constraint to move
/// * `schedulable_slots` - The slots that the constraint are allowed or preferred to be moved into
///
/// # Returns
/// * Vec<ChangeType> - The changes made during the execution of the strategy
/// * None - If no compatible slot was found to move the constraint into
pub fn execute_move_strategy(
    schedule: &mut Schedule,
    constraint_id: u32,
    constraint_duration: u8,
    schedulable_slots: SchedulableSlots,
) -> Option<Vec<ChangeType>> {
    let alternative_slot =
        schedule.get_free_slot_for_constraint(constraint_duration, &schedulable_slots);

    if let Some(slot) = alternative_slot {
        // TODO: Better error management
        let previous_slot = schedule
            .move_constraint(constraint_id, constraint_duration, slot)
            .unwrap();
        return Some(vec![
            ChangeType::Unscheduled(constraint_id, constraint_duration, previous_slot),
            ChangeType::Scheduled(constraint_id),
        ]);
    }

    None
}
pub fn execute_swap_strategy(
    constraint_id: u32,
    constraint_duration: u8,
    constraint_store: &mut ConstraintStore,
    schedule: &mut Schedule,
) -> Option<Vec<ChangeType>> {
    let swappable_constraint = constraint_store.find_swappable_scheduled_constraint(
        constraint_id,
        constraint_duration,
        schedule,
    )?;

    // TODO: Better error handling
    // TODO: FIx the bug where the first constraint might not be big enough for the compatible
    // constraint to fit into
    let previous_slot = schedule.unschedule_constraint(constraint_id).unwrap();
    let freed_slot = schedule
        .unschedule_constraint(swappable_constraint.id)
        .unwrap();

    schedule.schedule_constraint(constraint_id, constraint_duration, &freed_slot);
    schedule.schedule_constraint(
        swappable_constraint.id,
        swappable_constraint.duration,
        &previous_slot,
    );

    vec![
        ChangeType::Unscheduled(constraint_id, constraint_duration, previous_slot),
        ChangeType::Unscheduled(
            swappable_constraint.id,
            swappable_constraint.duration,
            freed_slot,
        ),
        ChangeType::Scheduled(constraint_id),
        ChangeType::Scheduled(swappable_constraint.id),
    ]
    .into()
}

/// Given a constraint, finds a slot for it (whether free or not), unschedules all constraints
/// under a duration starting from that slot and schedules the constraint in that slot
///
/// # Arguments
/// * `schedule` - The scheduled where the constraint should be schedule in
/// * `constraint_id` - The id of the constraint to schedule
/// * `constraint_duration` - The duration of the constraint to schedule
///
/// # Returns
/// * Vec<ChangeType> - The changes made whilst executing the strategy
/// * None - If the strategy was not able to be executed (This should not happen for this type of
/// strategy)
pub fn execute_substitute_strategy(
    schedule: &mut Schedule,
    constraint_id: u32,
    constraint_duration: u8,
) -> Option<Vec<ChangeType>> {
    let slot = schedule.choose_slot_for_constraint(constraint_duration);
    let unscheduled_constraints =
        schedule.unschedule_constraints_under_duration_from_slot(&slot, constraint_duration);

    let mut changes_made: Vec<ChangeType> = unscheduled_constraints
        .iter()
        .map(|c| ChangeType::Unscheduled(c.0, c.1, c.2.clone()))
        .collect();

    schedule.schedule_constraint(constraint_id, constraint_duration, &slot);
    changes_made.push(ChangeType::Scheduled(constraint_id));
    Some(changes_made)
}

/// Executes the strategy of scheduling a constraint at a specified slot
///
/// # Arguments
/// * `schedule` - The schedule in which to schedule the constraint in
/// * `constraint_id` - The id of the constraint to schedule
/// * `constraint_duration` - The duration of the constraint to schedule
/// * `slot_to_schedule` - The slot to schedule the constraint in
///
/// # Returns
/// *` vec<ChangeType>` - The changes made whilst executing the constraint
/// * None - If the strategy was not able to be executed (This should not happen for this type of
/// strategy)
pub fn execute_schedule_strategy(
    schedule: &mut Schedule,
    constraint_id: u32,
    constraint_duration: u8,
    slot_to_schedule: Slot,
) -> Option<Vec<ChangeType>> {
    schedule.schedule_constraint(constraint_id, constraint_duration, &slot_to_schedule);
    Some(vec![ChangeType::Scheduled(constraint_id)])
}
