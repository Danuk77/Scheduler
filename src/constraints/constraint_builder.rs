use crate::constraints::{Constraint, ConstraintPriority};

/// A simple builder class used for building constraints
/// As part of building the constraint, its relevant penalty function is generated.
/// The generated penalty function depends on the parameters used when building the constraint.
struct ConstraintBuilder {
    constraint_id: Option<u32>,
    constraint_name: Option<String>,
    priority: Option<ConstraintPriority>,
    duration: Option<u8>,
    gap: Option<u8>,
    allowed_slots: Option<Vec<(u8, u8)>>,
    preferred_slots: Option<Vec<(u8, u8)>>,
}

impl ConstraintBuilder {
    pub fn new() -> ConstraintBuilder {
        ConstraintBuilder {
            constraint_id: None,
            constraint_name: None,
            priority: None,
            duration: None,
            gap: None,
            allowed_slots: None,
            preferred_slots: None
        }
    }

    /// Set the id of the constraint.
    /// Use the same id between two constraints to identify the same type of constraint
    /// (You may wish to do this to impose various penalties such as gap based penalties between
    /// two constraints of the same type)
    pub fn set_id(mut self, id: u32) -> Self {
        self.constraint_id = Some(id);
        self
    }

    /// Set the name of the constraint
    pub fn set_name(mut self, name: String) -> Self {
        self.constraint_name = Some(name);
        self
    }

    /// Set the priority of the constraint
    pub fn set_priority(mut self, priority: ConstraintPriority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Set the duration of the constraint
    /// The duration is measured by how many slots the particular constraint takes
    ///
    /// i.e. A 2 hour constraint will take 4 slots (each slot is 30 minutes)
    pub fn set_duration(mut self, duration: u8) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set the gap (in terms of slots) between 2 instances of the same constraint
    /// i.e. Assume two
    pub fn set_gap(mut self, gap: u8) -> Self {
        self.gap = Some(gap);
        self
    }

    /// Set the slots this constraint is allowed to take
    /// # Arguments
    /// slots Vec<(u8, u8)> - Used for specifying the the day and the index of the slot
    pub fn set_allowed_slots(mut self, slots: Vec<(u8, u8)>) -> Self {
        self.allowed_slots = Some(slots);
        self
    }

    /// Set the slots this constraint should prefer to take (The scheduled slot may not always be
    /// at a preferred slot)
    /// # Arguments
    /// slots Vec<(u8, u8)> - Used for specifying the the day and the index of the slot
    pub fn set_preferred_slots(mut self, slots: Vec<(u8, u8)>) -> Self {
        self.preferred_slots = Some(slots);
        self
    }
}
