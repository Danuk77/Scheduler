use serde::{Deserialize, Serialize};

use crate::{
    constraints::{
        constraint_store::ConstraintStore,
        penalties::{
            calculate_allowed_slots_based_penalty, calculate_gap_based_penalty,
            calculate_preferred_slots_based_penalty, calculate_presence_based_penalty,
        },
        penalty::Penalty,
    },
    schedule::{Schedule, Slot},
};

pub mod constraint_builder;
pub mod constraint_store;
pub mod penalties;
pub mod penalty;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Constraint {
    pub name: String,
    pub id: u32,
    pub penalties: Vec<Penalty>,
    pub priority: ConstraintPriority,
    pub duration: u8,
    pub gap: Option<u16>,
    pub allowed_slots: Option<Vec<Slot>>,
    pub preferred_slots: Option<Vec<Slot>>,
}

impl Constraint {
    /// Calculates the total penalty for the constraint
    ///
    /// # Returs
    /// The calculated penalty
    pub fn calculate_penalty(
        &self,
        schedule: &Schedule,
        constraint_store: &ConstraintStore,
    ) -> u32 {
        let mut total_penalty: u32 = 0;

        for penalty in &self.penalties {
            match penalty {
                Penalty::Presence => {
                    total_penalty += calculate_presence_based_penalty(self, schedule)
                }
                Penalty::AllowedSlots => {
                    total_penalty += calculate_allowed_slots_based_penalty(self, schedule)
                }
                Penalty::PreferredSlots => {
                    total_penalty += calculate_preferred_slots_based_penalty(self, schedule)
                }
                Penalty::Gap => {
                    total_penalty += calculate_gap_based_penalty(self, schedule, constraint_store)
                }
            }
        }

        total_penalty
    }

    /// Calculates the penalties incurred by a constraint under a certain schedule
    ///
    /// Returns each of the penalties incurred by type
    ///
    /// # Arguments
    /// * `schedule` - The schedule the penalties must be evaluated under
    /// * `constraint_store` - The constraint store containing all the constraints
    ///
    /// # Returns
    /// * `Vec<(
    ///     Penalty - The type of incurred penalty
    ///     u32 - The value of the incurred penalty under that type
    /// )>`
    pub fn calculate_detailed_penalty(
        &self,
        schedule: &Schedule,
        constraint_store: &ConstraintStore,
    ) -> Vec<(Penalty, u32)> {
        let mut penalties: Vec<(Penalty, u32)> = Vec::new();

        for penalty in &self.penalties {
            match penalty {
                Penalty::Presence => penalties.push((
                    Penalty::Presence,
                    calculate_presence_based_penalty(self, schedule),
                )),
                Penalty::AllowedSlots => penalties.push((
                    Penalty::AllowedSlots,
                    calculate_allowed_slots_based_penalty(self, schedule),
                )),
                Penalty::PreferredSlots => penalties.push((
                    Penalty::PreferredSlots,
                    calculate_preferred_slots_based_penalty(self, schedule),
                )),
                Penalty::Gap => penalties.push((
                    Penalty::Gap,
                    calculate_gap_based_penalty(self, schedule, constraint_store),
                )),
            }
        }

        penalties
    }
}
