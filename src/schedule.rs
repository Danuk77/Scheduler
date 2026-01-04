use std::{array, usize};

use crate::constraints::Constraint;

pub struct Schedule {
    pub grid: [[Option<u32>; 48]; 7], // Option<u32> stores the id of the constraint, or None if
                                  // nothing is scheduled
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            grid: array::from_fn(|_| array::from_fn(|_| None)),
        }
    }

    pub fn get_slot_for_constraint(&self, constraint: &Constraint) -> Option<(u8, u8)> {
        match &constraint.allowed_slots {
            Some(slots) => {
                return slots
                    .iter()
                    .find(|slot| is_duration_free(slot, constraint.duration, &self.grid))
                    .cloned();
            }
            None => {
                return self.find_free_slot(constraint);
            }
        }
    }

    fn find_free_slot(&self, constraint: &Constraint) -> Option<(u8, u8)> {
        let mut slot_index: (u8, u8) = (0, 0);
        loop {
            let day_slots = self.grid.get(slot_index.0 as usize)?;
            match day_slots.get(slot_index.1 as usize) {
                None => {
                    slot_index.0 += 1;
                    slot_index.1 = 0;
                    continue;
                }
                Some(_) => {
                    if is_duration_free(&slot_index, constraint.duration, &self.grid) {
                        return Some(slot_index);
                    } else {
                        // TODO: The index should be incremented by the index of the scheduled
                        // constraint
                        slot_index.1 += 1;
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
    pub fn schedule_constraint(&mut self, constraint: &mut Constraint, slot: (u8, u8)) {
        for i in 0..constraint.duration {
            self.grid[slot.0 as usize][(slot.1 + i) as usize] = Some(constraint.id);
        }

        constraint.schedule_for_slot(slot);
    }
}

/// Given a slot index, a duration and a grid, checks if the slot specified by the index is free
/// for the specified duration
///
/// # Arguments
/// * slot_index - The index of the slot
/// * duration - The duration to check for
/// * grid - The grid containing the schedule
///
/// Does not panic
///
/// # Returns
/// * true if it is free
/// * false otherwise
fn is_duration_free(slot_index: &(u8, u8), duration: u8, grid: &[[Option<u32>; 48]; 7]) -> bool {
    for i in 0..duration {
        if !is_slot_free(&(slot_index.0, slot_index.1 + i), grid) {
            return false;
        }
    }

    true
}

/// Given a slot index and a grid, checks if the slot specified by the index is free
///
/// # Arguments
/// * slot_index - The index of the slot
/// * grid - The grid containing the schedule
///
/// Does not panic
///
/// # Returns
/// * true if it is free
/// * false otherwise
fn is_slot_free(slot_index: &(u8, u8), grid: &[[Option<u32>; 48]; 7]) -> bool {
    if let Some(None) = grid
        .get(slot_index.0 as usize)
        .and_then(|day| day.get(slot_index.1 as usize))
    {
        return true;
    }

    false
}
