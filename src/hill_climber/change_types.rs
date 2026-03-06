use crate::schedule::{Schedule, Slot};

pub enum ChangeType {
    Move(u32, Slot, Slot, u8), // (constraint id, new slot, previous slot, constraint duration)
    Scheduled(u32, u8),        // (constraint id, duration)
    Subtituted((u32, u8), (u32, u8)), // ((new constraint, duration), (previous constraint, duration))
}

impl ChangeType {
    pub fn revert_change(&self, schedule: &mut Schedule) {
        match self {
            ChangeType::Move(constraint_id, _, previous_slot, constraint_duration) => {
                schedule.unschedule_constraint(*constraint_id, *constraint_duration);
                schedule.schedule_constraint(*constraint_id, *constraint_duration, previous_slot);
            }
            ChangeType::Scheduled(constraint_id, constraint_duration) => {
                schedule.unschedule_constraint(*constraint_id, *constraint_duration);
            }
            ChangeType::Subtituted(
                (new_constraint, new_constraint_duration),
                (old_constraint, old_constraint_duration),
            ) => {
                let freed_slot = schedule
                    .unschedule_constraint(*new_constraint, *new_constraint_duration)
                    .expect("Unexpected logic error when unscheduling scheduled constraint");
                schedule.schedule_constraint(
                    *old_constraint,
                    *old_constraint_duration,
                    &freed_slot,
                );
            }
        }
    }
}
