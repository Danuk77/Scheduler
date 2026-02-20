pub mod errors;

use std::collections::HashMap;
use std::{array, usize};

use crate::schedule::errors::ScheduleError;

#[derive(Clone)]
pub struct Slot {
    pub day: u8,
    pub window: u8,
}

#[derive(Clone)]
pub struct Schedule {
    pub grid: [[Option<u32>; 48]; 7], // Option<u32> stores the id of the constraint, or None if
    // nothing is scheduled
    scheduled_constraints: HashMap<u32, Slot>,
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            grid: array::from_fn(|_| array::from_fn(|_| None)),
            scheduled_constraints: HashMap::new(),
        }
    }

    /// Finds a slot compatible to schedule a constraint
    /// # Arguments
    /// * constraint_duration - The duration of the constraint to find a slot
    /// * allowed_slots - The slots the constraint is allowed to be in
    ///
    /// # Returns
    /// * Slot - The slot representing the starting point to schedule the constraint to
    /// * None - If no slot exists for the specified duration
    pub fn get_slot_for_constraint(
        &self,
        constraint_duration: u8,
        allowed_slots: &Option<Vec<Slot>>,
    ) -> Option<Slot> {
        if let Some(slots) = allowed_slots {
            return slots
                .iter()
                .find(|slot| self.is_duration_free(slot, constraint_duration))
                .map(|free_slot| Slot {
                    day: free_slot.day,
                    window: free_slot.window,
                });
        } else {
            return self.find_free_slot(constraint_duration);
        }
    }

    fn find_free_slot(&self, constraint_duration: u8) -> Option<Slot> {
        let mut s: Slot = Slot { day: 0, window: 0 };
        loop {
            let day_slots = self.grid.get(s.day as usize)?;
            match day_slots.get(s.window as usize) {
                None => {
                    s.day += 1;
                    s.window = 0;
                    continue;
                }
                Some(_) => {
                    if self.is_duration_free(&s, constraint_duration) {
                        return Some(s);
                    } else {
                        // TODO: The index should be incremented by the duration of the scheduled
                        // constraint
                        s.window += 1;
                    }
                }
            }
        }
    }

    /// Schedules a constraint to a given slot
    ///
    /// # Panics
    /// If the specified slot is out of bounds for the schedule, it will panic
    ///
    /// # Arguments
    /// * constraint - The constraint to schedule
    /// * slot - The slot to schedule the constraint at
    pub fn schedule_constraint(
        &mut self,
        constraint_id: u32,
        constraint_duration: u8,
        slot: &Slot,
    ) {
        for i in 0..constraint_duration {
            self.grid[slot.day as usize][(slot.window + i) as usize] = Some(constraint_id);
        }

        self.scheduled_constraints
            .insert(constraint_id, slot.clone());
    }

    /// Unschedules a constraint from its scheduled slot in the schedule
    ///
    /// # Arguments
    /// * constraint_id - The id of the constraint to unschedule
    /// * constraint_duratin - The duration of the constraint
    ///
    /// # Returns
    /// * Slot - The freed up slot after unscheduling
    /// * ScheduleError - If trying to unschedule a constraint that is not scheduled
    pub fn unschedule_constraint(
        &mut self,
        constraint_id: u32,
        constraint_duration: u8,
    ) -> Result<Slot, ScheduleError> {
        let Some(scheduled_slot) = self.scheduled_constraints.remove(&constraint_id) else {
            return Err(ScheduleError::ConstraintNotScheduled(constraint_id));
        };

        for i in 0..constraint_duration {
            self.grid[scheduled_slot.day as usize][(scheduled_slot.window + i) as usize] = None;
        }

        Ok(scheduled_slot)
    }

    /// Returns whether a specific constraint is scheduled or not
    ///
    /// # Returns
    /// * bool - Whether scheduled or not
    pub fn is_constraint_scheduled(&self, constraint_id: u32) -> bool {
        self.scheduled_constraints.contains_key(&constraint_id)
    }

    /// Returns `true` if a range of slots is either empty or occupied
    /// exclusively by the specified constraint.
    ///
    /// # Arguments
    /// * `constraint_id` - The ID to allow; any other ID found will cause this to return `false`.
    /// * `duration` - The number of consecutive slots to check.
    /// * `start_slot` - The initial slot (day and window) to begin the check.
    ///
    /// # Returns
    /// * `true` if all slots in the range are `None` or match `constraint_id`.
    /// * `false` if the range exceeds the grid bounds or contains a different ID.
    pub fn is_duration_free_or_owned_by(
        &self,
        constraint_id: &u32,
        duration: u8,
        start_slot: &Slot,
    ) -> bool {
        let Some(day) = self.grid.get(start_slot.day as usize) else {
            return false;
        };

        for i in 0..duration {
            let Some(window) = day.get((start_slot.window + i) as usize) else {
                return false;
            };

            if let Some(scheduled_constraint_id) = window {
                if scheduled_constraint_id != constraint_id {
                    return false;
                }
            }
        }

        return true;
    }

    /// Given a slot index, a duration and a grid, checks if the slot specified by the index is free
    /// for the specified duration
    ///
    /// # Arguments
    /// * slot_index - The index of the slot
    /// * duration - The duration to check for
    ///
    /// Does not panic
    ///
    /// # Returns
    /// * true if it is free
    /// * false otherwise
    pub fn is_duration_free(&self, slot: &Slot, duration: u8) -> bool {
        for i in 0..duration {
            if !self.is_slot_free(&(slot.day, slot.window + i)) {
                return false;
            }
        }

        true
    }

    /// Given a slot index and a grid, checks if the slot specified by the index is free
    ///
    /// # Arguments
    /// * slot_index - The index of the slot
    ///
    /// Does not panic
    ///
    /// # Returns
    /// * true if it is free
    /// * false otherwise
    pub fn is_slot_free(&self, slot_index: &(u8, u8)) -> bool {
        if let Some(None) = self
            .grid
            .get(slot_index.0 as usize)
            .and_then(|day| day.get(slot_index.1 as usize))
        {
            return true;
        }

        false
    }
}
