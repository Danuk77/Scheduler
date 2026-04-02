pub mod errors;

use csv::Writer;
use rand::seq::IteratorRandom;
use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::{array, usize};

use crate::constraints::constraint_store::ConstraintStore;
use crate::hill_climber::make_small_change::SchedulableSlots;
use crate::schedule::errors::ScheduleError;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Slot {
    pub day: u8,
    pub window: u8,
}

#[derive(Clone, Debug, Serialize)]
pub struct Schedule {
    #[serde(with = "serde_arrays")]
    pub grid: [[Option<u32>; 48]; 7], // Option<u32> stores the id of the constraint, or None if nothing is scheduled
    #[serde(skip_serializing)]
    scheduled_constraints: HashMap<u32, (Slot, u8)>, // K = constraint_id, V = (slot, scheduled_duration)
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            grid: array::from_fn(|_| array::from_fn(|_| None)),
            scheduled_constraints: HashMap::new(),
        }
    }

    /// Finds a free compatible slot to schedule a constraint
    ///
    /// /// TODO: Add a way to make allowed slots and preferred slots work togeather,
    /// make it so that if an allowed slot is also preferred, it has higher presidence to be chosen
    /// # Arguments
    /// * constraint_duration - The duration of the constraint to find a slot
    /// * allowed_slots - The slots the constraint is allowed to be in
    ///
    /// # Returns
    /// * Slot - The slot representing the starting point to schedule the constraint to
    /// * None - If no slot exists for the specified duration
    pub fn get_free_slot_for_constraint(
        &self,
        constraint_duration: u8,
        schedulable_slots: &SchedulableSlots,
    ) -> Option<Slot> {
        if let Some(slots) = &schedulable_slots.allowed_slots {
            return slots
                .iter()
                .filter(|slot| self.is_duration_free(slot, constraint_duration))
                .choose(&mut rng())
                .cloned();
        }

        if let Some(slots) = &schedulable_slots.preferred_slots {
            if let Some(preferred_slot) = slots
                .iter()
                .filter(|slot| self.is_duration_free(slot, constraint_duration))
                .choose(&mut rng())
            {
                return Some(preferred_slot.clone());
            }
        }

        return self.find_free_slot(constraint_duration);
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

    /// Chooses a slot for the given constraint's schedulable slots (or any other slots) regardless of whether the slots
    /// are free or not
    ///
    /// # Arguments
    /// * `constraint_duration` - The duration of the constraint to choose a slot for
    ///
    /// # Returns
    /// * `Slot` - If a slot was found with a duration enough for the constraint
    /// * `None` - Otherwise
    pub fn choose_slot_for_constraint(&self, constraint_duration: u8) -> Slot {
        let date = rng().random_range(0..=6);
        let window = rng().random_range(0..=(48 - (constraint_duration * 2)));

        Slot {
            day: date,
            window: window,
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
            .insert(constraint_id, (slot.clone(), constraint_duration));
    }

    /// Unschedules all constraints starting from the specified slot until the end of the specified
    /// duration
    ///
    /// # Arguments
    /// * `slot` - The starting slot to unschedule constraints from
    /// * `duration` - The duration to unschedule constraints from
    pub fn unschedule_constraints_under_duration_from_slot(
        &mut self,
        slot: &Slot,
        duration: u8,
    ) -> Vec<(u32, u8, Slot)> {
        let mut unscheduled_constraints: Vec<(u32, u8, Slot)> = Vec::new();
        let constraints_in_day = self.grid.get(slot.day as usize).unwrap();

        let mut i = 0;
        while i < duration {
            if let Some(constraint_id) = constraints_in_day[i as usize + slot.window as usize] {
                let (slot, constraint_duration) = self
                    .scheduled_constraints
                    .get(&constraint_id)
                    .expect("Could not get scheduled constraint");
                unscheduled_constraints.push((constraint_id, *constraint_duration, slot.clone()));
                i += constraint_duration;
            } else {
                i += 1;
            }
        }

        unscheduled_constraints.iter().for_each(|(id, _, _)| {
            self.unschedule_constraint(*id)
                .expect("LOGIC ERROR: Could not unschedule scheduled constraint");
        });

        unscheduled_constraints
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
    pub fn unschedule_constraint(&mut self, constraint_id: u32) -> Result<Slot, ScheduleError> {
        let Some((scheduled_slot, constraint_duration)) =
            self.scheduled_constraints.remove(&constraint_id)
        else {
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

    /// Returns the scheduled slot for the given constraint id
    ///
    /// # Arguments
    /// * `constraint_id` - The id of the constraint to return the scheduled slot for
    ///
    /// # Returns
    /// * `&Slot` - The scheduled slot
    /// * `None` - If the constraint is not scheduled
    pub fn get_scheduled_slot_for_constraint(&self, constraint_id: u32) -> Option<&Slot> {
        match self.scheduled_constraints.get(&constraint_id) {
            None => None,
            Some((slot, _)) => Some(slot),
        }
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

    /// Exports the schedule to a csv file
    ///
    /// # Arguments
    /// * `file_name` - The name of the file to export the csv to
    pub fn export_to_csv(
        &self,
        file_name: String,
        constraint_store: &ConstraintStore,
    ) -> Result<(), Box<dyn Error>> {
        let mut csv_writer = Writer::from_path(file_name)?;
        csv_writer.write_record(&[
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ])?;
        for i in 0..48 {
            for j in 0..7 {
                let constraint_name = match self.grid[j][i]{
                   None => String::from("Free"),
                   Some(constraint_id) => constraint_store
                       .get_constraint(constraint_id)
                       .expect("Unexpected logic error when exporting to csv. Could not find constraint in constraint store")
                       .name
                       .clone()
                };
                csv_writer.write_field(constraint_name)?;
            }
            csv_writer.write_record(None::<&[u8]>)?;
        }

        csv_writer.flush()?;

        Ok(())
    }
}
