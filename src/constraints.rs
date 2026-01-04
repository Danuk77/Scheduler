use crate::constraints::{
    penalties::{
        calculate_allowed_slots_based_penalty, calculate_gap_based_penalty,
        calculate_preferred_slots_based_penalty, calculate_validity_based_penalty,
    },
    penalty::Penalty,
};

pub mod constraint_builder;
pub mod penalties;
pub mod penalty;

#[derive(Clone)]
pub enum ConstraintPriority {
    High,
    Low,
}

/// Represents a scheduling constraint with associated penalties and requirements.
///
/// # Fields
///
/// * `name` - Human-readable name for the constraint
/// * `id` - Unique identifier for the constraint
/// * `penalties` - Collection of penalties applied when this constraint is violated
/// * `priority` - Importance level of this constraint in the scheduling system
/// * `duration` - Length of time required for this constraint (in time units)
/// * `gap` - Optional minimum gap required between scheduling slots
/// * `allowed_slots` - Optional whitelist of permitted time slots as (day, slot) tuples (Only the
/// start slot is specified)
/// * `preferred_slots` - Optional list of preferred time slots as (day, slot) tuples (Only the
/// start slot is specified)
/// * `scheduled_slot` - The currently assigned time slot, if scheduled
pub struct Constraint {
    pub name: String,
    pub id: u32,
    pub penalties: Vec<Penalty>,
    pub priority: ConstraintPriority,
    pub duration: u8,
    pub gap: Option<u8>,
    pub allowed_slots: Option<Vec<(u8, u8)>>,
    pub preferred_slots: Option<Vec<(u8, u8)>>,

    scheduled_slot: Option<(u8, u8)>,
}

impl Constraint {
    /// Schedules the constraint for the specified slot
    ///
    /// # Arguments
    /// * slot - A tuple (day, slot) specifying the date the starting slot to schedule the
    /// constraint
    pub fn schedule_for_slot(&mut self, slot: (u8, u8)) {
        self.scheduled_slot = Some(slot);
    }

    /// Method to calculate the total penalty for the constraint
    ///
    /// # Returs
    /// The calculated penalty
    pub fn calculate_penalty(&self) -> u32 {
        let mut total_penalty: u32 = 0;

        for penalty in &self.penalties {
            match penalty {
                Penalty::Validity => total_penalty += calculate_validity_based_penalty(self),
                Penalty::AllowedSlots => {
                    total_penalty += calculate_allowed_slots_based_penalty(self)
                }
                Penalty::PreferredSlots => {
                    total_penalty += calculate_preferred_slots_based_penalty(self)
                }
                Penalty::Gap => total_penalty += calculate_gap_based_penalty(self),
            }
        }

        total_penalty
    }
}
