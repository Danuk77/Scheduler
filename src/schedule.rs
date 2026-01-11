use std::collections::HashMap;
use std::{array, usize};

use crate::constraints::Constraint;

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

    pub fn get_slot_for_constraint(&self, constraint: &Constraint) -> Option<Slot> {
        match &constraint.allowed_slots {
            Some(slots) => {
                return slots
                    .iter()
                    .find(|slot| is_duration_free(slot, constraint.duration, &self.grid))
                    .map(|free_slot| Slot {
                        day: free_slot.0,
                        window: free_slot.1,
                    });
            }
            None => {
                return self.find_free_slot(constraint);
            }
        }
    }

    fn find_free_slot(&self, constraint: &Constraint) -> Option<Slot> {
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
                    if is_duration_free(&s, constraint.duration, &self.grid) {
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
    pub fn schedule_constraint(&mut self, constraint: &Constraint, slot: Slot) {
        for i in 0..constraint.duration {
            self.grid[slot.day as usize][(slot.window + i) as usize] = Some(constraint.id);
        }

        self.scheduled_constraints.insert(constraint.id, slot);
    }

    /// Returns whether a specific constraint is scheduled or not
    ///
    /// # Returns
    /// * bool - Whether scheduled or not
    pub fn is_constraint_scheduled(&self, constraint_id: u32) -> bool {
        self.scheduled_constraints.contains_key(&constraint_id)
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
fn is_duration_free(slot: &Slot, duration: u8, grid: &[[Option<u32>; 48]; 7]) -> bool {
    for i in 0..duration {
        if !is_slot_free(&(slot.day, slot.window + i), grid) {
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
