use anyhow::{Context, Result, anyhow};

use crate::{
    constraints::{Constraint, ConstraintPriority, penalty::Penalty},
    schedule::Slot,
};

#[allow(dead_code)]
/// A simple builder class used for building constraints
/// As part of building the constraint, its relevant penalty function is generated.
/// The generated penalty function depends on the parameters used when building the constraint.
pub struct ConstraintBuilder {
    id_counter: u32,
    constraint_name: Option<String>,
    priority: Option<ConstraintPriority>,
    duration: Option<u8>,
    gap: Option<u16>,
    allowed_slots: Option<Vec<Slot>>,
    preferred_slots: Option<Vec<Slot>>,
    penalties: Vec<Penalty>,
    frequency: Option<u8>,
}

#[allow(dead_code)]
impl ConstraintBuilder {
    pub fn new() -> ConstraintBuilder {
        ConstraintBuilder {
            id_counter: 0,
            constraint_name: None,
            priority: None,
            duration: None,
            gap: None,
            allowed_slots: None,
            preferred_slots: None,
            penalties: Vec::new(),
            frequency: None,
        }
    }

    /// Set the name of the constraint
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.constraint_name = Some(name);
        self
    }

    /// Set the priority of the constraint
    pub fn set_priority(&mut self, priority: ConstraintPriority) -> &mut Self {
        self.priority = Some(priority);
        self
    }

    /// Set how many times this constraint needs to be scheduled
    ///
    /// Setting a frequency greater than 1 results in multiple constraints being built with the
    /// same attributes
    pub fn set_frequency(&mut self, frequency: u8) -> Result<&mut Self, String> {
        if frequency <= 0 {
            return Err("The frequency value must be greater than 0".to_string());
        };

        self.frequency = Some(frequency);
        Ok(self)
    }

    /// Set the duration of the constraint
    /// The duration is measured by how many slots the particular constraint takes
    ///
    /// i.e. A 2 hour constraint will take 4 slots (each slot is 30 minutes)
    pub fn set_duration(&mut self, duration: u8) -> &mut Self {
        self.duration = Some(duration);
        self.penalties.push(Penalty::Presence);
        self
    }

    /// Set the gap (in terms of slots) between two schedulings of the same constraint type
    ///
    /// Constraints with the same name are considered of the same type
    ///
    /// # Arguments
    /// * `gap` - The number of slots gap
    ///
    /// # Returns
    /// * `ConstraintBuilder` - The builder class with the gap configured
    pub fn set_gap(&mut self, gap: u16) -> &mut Self {
        self.gap = Some(gap);
        self.penalties.push(Penalty::Gap);
        self
    }

    /// Set the slots this constraint is allowed to take
    /// # Arguments
    /// slots Vec<(u8, u8)> - Used for specifying the the day and the index of the slot
    pub fn set_allowed_slots(&mut self, slots: Vec<Slot>) -> &mut Self {
        self.allowed_slots = Some(slots);
        self.penalties.push(Penalty::AllowedSlots);
        self
    }

    /// Set the slots this constraint should prefer to take (The scheduled slot may not always be
    /// at a preferred slot)
    /// # Arguments
    /// slots Vec<(u8, u8)> - Used for specifying the the day and the index of the slot
    pub fn set_preferred_slots(&mut self, slots: Vec<Slot>) -> &mut Self {
        self.preferred_slots = Some(slots);
        self.penalties.push(Penalty::PreferredSlots);
        self
    }

    /// Builds and returns the constraint specified
    /// Clears the content of the builder after the constraint is built
    ///
    /// Does not panic
    ///
    /// Returns Err when a required field is not or validation fails
    pub fn build(&mut self) -> Result<Vec<Constraint>> {
        if self.penalties.is_empty() {
            return Err(anyhow!(
                "Empty constraint, please ensure criteria is specified for the constraint"
            ));
        }
        let constraints: Result<Vec<Constraint>> = (0..self.frequency.unwrap_or(1))
            .map(|_| {
                self.id_counter += 1;
                Ok(Constraint {
                    name: self
                        .constraint_name
                        .clone()
                        .context("Please ensure the name is set for constraint")?,
                    id: self.id_counter,
                    penalties: self.penalties.clone(),
                    priority: self
                        .priority
                        .clone()
                        .context("Please ensure the priority is set for the constraint")?,
                    duration: self
                        .duration
                        .context("Please ensure the duration is specified for the constraint")?,
                    gap: self.gap,
                    allowed_slots: self.allowed_slots.clone(),
                    preferred_slots: self.preferred_slots.clone(),
                })
            })
            .collect();

        self.clear();
        constraints
    }

    /// Clears the state of the builder
    ///
    /// Typically called after a constraint has been created using `build`
    pub fn clear(&mut self) {
        self.constraint_name = None;
        self.priority = None;
        self.duration = None;
        self.gap = None;
        self.allowed_slots = None;
        self.preferred_slots = None;
        self.penalties = Vec::new();
        self.frequency = None;
    }
}
