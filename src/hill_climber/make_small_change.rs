use log::debug;
use rand::{rng, seq::IndexedRandom};

use crate::{
    constraints::constraint_store::ConstraintStore,
    hill_climber::{
        OptimisationStats,
        change_types::ChangeType,
        optimisation_strategies::{
            OptimisationStrategy, execute_move_strategy, execute_schedule_strategy,
            execute_substitute_strategy, execute_swap_strategy,
        },
    },
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
    stats: &mut OptimisationStats,
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
            constraints,
            constraint_id,
            constraint_duration,
            schedulabe_slots_for_constraint,
            schedule,
            stats,
        ));
    } else {
        return Ok(handle_unscheduled_constraint(
            constraint_id,
            constraint_duration,
            schedulabe_slots_for_constraint,
            schedule,
            stats,
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
    constraint_store: &mut ConstraintStore,
    constraint_id: u32,
    constraint_duration: u8,
    schedulable_slots: SchedulableSlots,
    schedule: &mut Schedule,
    stats: &mut OptimisationStats,
) -> Option<Vec<ChangeType>> {
    // TODO: Add comment about what is going here
    // Maybe move it into a separate function
    let (option, _) = [
        (OptimisationStrategy::MOVE, 3),
        (OptimisationStrategy::UNSCHEDULE, 1),
        (OptimisationStrategy::SWAP, 1),
    ]
    .choose_weighted(&mut rng(), |s| s.1)
    .unwrap();

    match option {
        OptimisationStrategy::MOVE => {
            stats.move_count += 1;
            return execute_move_strategy(
                schedule,
                constraint_id,
                constraint_duration,
                schedulable_slots,
            );
        }
        OptimisationStrategy::UNSCHEDULE => {
            stats.unscheduling_scheduled_count += 1;

            // First unschedule the constraint from its existing slot
            let freed_slot = schedule.unschedule_constraint(constraint_id).unwrap();
            let mut changes_made =
                execute_substitute_strategy(schedule, constraint_id, constraint_duration)?;
            changes_made.insert(
                0,
                ChangeType::Unscheduled(constraint_id, constraint_duration, freed_slot.clone()),
            );
            Some(changes_made)
        }
        OptimisationStrategy::SWAP => {
            stats.swap_count += 1;
            return execute_swap_strategy(
                constraint_id,
                constraint_duration,
                constraint_store,
                schedule,
            );
        }
        _ => panic!(
            "Logic error: Should not reach here. The specified strategy is not valid for scheduled constraint"
        ),
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
    stats: &mut OptimisationStats,
) -> Option<Vec<ChangeType>> {
    match schedule.get_free_slot_for_constraint(constraint_duration, &schedulable_slots) {
        Some(slot) => {
            stats.schedule_count += 1;
            return execute_schedule_strategy(schedule, constraint_id, constraint_duration, slot);
        }
        None => {
            stats.unscheduling_unscheduled_count += 1;
            return execute_substitute_strategy(schedule, constraint_id, constraint_duration);
        }
    }
}
