use crate::{
    constraints::constraint_store::ConstraintStore, hill_climber::change_types::ChangeType,
    schedule::Schedule,
};

pub fn make_small_change(
    constraints: &mut ConstraintStore,
    schedule: &mut Schedule,
) -> Option<ChangeType> {
    let constraint = constraints.get_constraint_for_adjustment()?;
    let constraint_id = constraint.id;
    let constraint_duration = constraint.duration;

    match schedule.is_constraint_scheduled(constraint_id) {
        true => {
            if let Some(allowed_slots) = &constraint.allowed_slots {
                if let Some(slot) = allowed_slots
                    .iter()
                    .find(|slot| schedule.is_duration_free(&slot, constraint_duration))
                {
                    let previous_slot = schedule
                        .unschedule_constraint(constraint)
                        .expect("Unexpected logic error");
                    schedule.schedule_constraint(constraint_id, constraint_duration, slot);
                    return Some(ChangeType::Move(previous_slot, slot.clone()));
                } else {
                    return None;
                }
            }

            if let Some(slot) = schedule.get_slot_for_constraint(constraint) {
                let previous_slot = schedule
                    .unschedule_constraint(constraint)
                    .expect("Unexpected logic error");
                schedule.schedule_constraint(constraint_id, constraint_duration, &slot);
                return Some(ChangeType::Move(previous_slot, slot));
            }

            // NOTE: This is not working as expected
            //if let Some(swappable_constraint) = constraints.find_swappable_scheduled_constraint(
            //    constraint_id,
            //    constraint_duration,
            //    schedule,
            //) {
            //    let freed_slot = schedule
            //        .unschedule_constraint(swappable_constraint)
            //        .expect("Unexpected error ocurred whilst unscheduling constraint");
            //    schedule.schedule_constraint(constraint_id, constraint_duration, &freed_slot);
            //    return Some(ChangeType::Subtituted(
            //        constraint_id,
            //        swappable_constraint.id,
            //    ));
            //}

            println!("Cannot optimise constraint (scheuled)");
            None
        }
        false => {
            if let Some(slot) = schedule.get_slot_for_constraint(constraint) {
                schedule.schedule_constraint(constraint_id, constraint_duration, &slot);
                return Some(ChangeType::Scheduled(constraint_id));
            }

            if let Some(swappable_constraint) = constraints.find_swappable_scheduled_constraint(
                constraint_id,
                constraint_duration,
                schedule,
            ) {
                let freed_slot = schedule
                    .unschedule_constraint(swappable_constraint)
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
    }
}
