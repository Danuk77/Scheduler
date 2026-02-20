use std::collections::HashMap;

use crate::{constraints::Constraint, schedule::Schedule};
use rand::prelude::*;
use rand::rng;

#[derive(Clone)]
/// Structure used for storing arbitrary constraints for optimisation
pub struct ConstraintStore {
    constraints: Vec<Constraint>,
    rng: ThreadRng,
}

impl ConstraintStore {
    /// Create a new empty constraint store
    pub fn new() -> Self {
        return ConstraintStore {
            constraints: Vec::new(),
            rng: rng(),
        };
    }

    /// Add a new constraint to the store
    ///
    /// # Arguments
    /// * `constraint` - The constraint to store
    ///
    /// # Returns
    /// None
    pub fn push(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    /// Retrieves a constraint for optimisation from the store
    ///
    /// Randomly selects a constraint weighted by the penalties specified
    ///
    /// # Arguments
    /// `penalties` - The penalties incurred by the constraints
    ///
    /// # Returns
    /// * `&Constraint` - The constraint selected for optimisation
    /// * `None` - If no constraint was selected
    pub fn get_constraint_for_optimisation(
        &mut self,
        penalties: &HashMap<u32, u32>,
    ) -> Option<&Constraint> {
        self.constraints
            .choose_weighted(&mut self.rng, |c| {
                *penalties.get(&c.id)
                    .expect("Error: encountered inconsistent constraint ids between penalty calculation and constraint store")
            })
            .ok()
    }

    pub fn find_swappable_scheduled_constraint(
        &self,
        constraint_id: u32,
        constraint_duration: u8,
        schedule: &Schedule,
    ) -> Option<&Constraint> {
        let compatible_constriants: Vec<&Constraint> = self
            .constraints
            .iter()
            .filter(|c| {
                (c.id != constraint_id && c.duration >= constraint_duration)
                    && schedule.is_constraint_scheduled(c.id)
            })
            .collect();

        compatible_constriants.choose(&mut rng()).copied()
    }

    pub fn get_constraint(&self, constraint_id: u32) -> Option<&Constraint> {
        return self.constraints.iter().find(|c| c.id == constraint_id);
    }
}

impl<'a> IntoIterator for &'a mut ConstraintStore {
    type Item = &'a mut Constraint;
    type IntoIter = std::slice::IterMut<'a, Constraint>;

    fn into_iter(self) -> Self::IntoIter {
        self.constraints.iter_mut()
    }
}

impl<'a> IntoIterator for &'a ConstraintStore {
    type Item = &'a Constraint;
    type IntoIter = std::slice::Iter<'a, Constraint>;

    fn into_iter(self) -> Self::IntoIter {
        self.constraints.iter()
    }
}
