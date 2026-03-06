use crate::schedule::{Schedule, Slot};

/// Represents the type of modification made to a [Schedule].
///
/// This enum is used to track changes so they can be rolled back
/// if an optimization step does not result in a better score.
pub enum ChangeType {
    /// A constraint was moved from one time slot to another.
    ///
    /// Fields: `(constraint_id, previous_slot, duration)`
    Move(u32, Slot, u8),

    /// A previously unscheduled constraint was added to the schedule.
    ///
    /// Fields: `(constraint_id, duration)`
    Scheduled(u32, u8),

    /// One constraint was removed and replaced by another in the same slot.
    ///
    /// Fields: `((new_constraint_id, new_duration), (previous_constraint_id, previous_duration))`
    Substituted((u32, u8), (u32, u8)),
}

impl ChangeType {
    /// Reverts the specific change on the provided [Schedule], restoring it to its previous state.
    ///
    /// # Arguments
    /// * `schedule` - A mutable reference to the [Schedule] to be modified.
    ///
    /// # Panics
    /// * For `Move` or `Scheduled` variants, it may panic if the `schedule` internal logic
    ///   cannot find the constraint to unschedule.
    /// * For the `Substituted` variant, it panics with a custom message if the `new_constraint`
    ///   is not found where the system expected it to be.
    pub fn revert_change(&self, schedule: &mut Schedule) {
        match self {
            ChangeType::Move(constraint_id, previous_slot, constraint_duration) => {
                schedule
                    .unschedule_constraint(*constraint_id, *constraint_duration)
                    .expect("Unexpected logic error when unscheduling scheduled constraint");
                schedule.schedule_constraint(*constraint_id, *constraint_duration, previous_slot);
            }

            ChangeType::Scheduled(constraint_id, constraint_duration) => {
                schedule
                    .unschedule_constraint(*constraint_id, *constraint_duration)
                    .expect("Unexpected logic error when unscheduling scheduled constraint");
            }

            ChangeType::Substituted(
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
