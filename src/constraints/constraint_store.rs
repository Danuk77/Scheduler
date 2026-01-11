use crate::{constraints::Constraint, schedule::Schedule};
use rand::rng;
use rand::{distr::weighted::WeightedIndex, prelude::*};

#[derive(Clone)]
pub struct ConstraintStore {
    constraints: Vec<Constraint>,
    penalties: Vec<u32>,
    rng: ThreadRng,
}

impl ConstraintStore {
    pub fn new() -> Self {
        return ConstraintStore {
            constraints: Vec::new(),
            penalties: Vec::new(),
            rng: rng(),
        };
    }

    pub fn calculate_penalties(&mut self, schedule: &Schedule) -> u32 {
        let mut total_penalty = 0;
        for (i, constraint) in self.constraints.iter().enumerate() {
            let constraint_penalty = constraint.calculate_penalty(schedule);
            self.penalties[i] = constraint_penalty;
            total_penalty += constraint_penalty;
        }

        total_penalty
    }

    pub fn push(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
        self.penalties.push(0);
    }

    pub fn get_constraint_for_adjustment(&mut self) -> Option<&Constraint> {
        let distribution = WeightedIndex::new(&self.penalties).ok()?;
        let index = distribution.sample(&mut self.rng);
        self.constraints.get(index)
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
