use crate::schedule::{Schedule, Slot};

/// Represents the type of modification made to a [Schedule].
///
/// This enum is used to track changes so they can be rolled back
/// if an optimization step does not result in a better score.
pub enum ChangeType {
    /// A constraint was scheduled for a specific duration.
    ///
    /// Fields: `(constraint_id, duration)`
    Scheduled(u32, u8),

    /// A constraint was unscheduled from a specific slot.
    ///
    /// Fields: `(constraint_id, duration, slot)`
    Unscheduled(u32, u8, Slot),
}

impl ChangeType {
    /// Reverts the specific change on the provided [Schedule], restoring it to its previous state.
    ///
    /// # Arguments
    /// * `schedule` - A mutable reference to the [Schedule] to be modified.
    pub fn revert_change(&self, schedule: &mut Schedule) {
        match self {
            ChangeType::Scheduled(constraint_id, constraint_duration) => {
                schedule
                    .unschedule_constraint(*constraint_id, *constraint_duration)
                    .expect("Unexpected logic error when unscheduling scheduled constraint");
            }

            ChangeType::Unscheduled(constraint_id, constraint_duration, slot) => {
                schedule.schedule_constraint(*constraint_id, *constraint_duration, slot);
            }
        }
    }
}
