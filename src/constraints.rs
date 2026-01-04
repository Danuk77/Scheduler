use crate::constraints::penalty::Penalty;

pub mod constraint_builder;
pub mod penalty;

#[derive(Clone)]
pub enum ConstraintPriority {
    High,
    Low,
}

pub struct Constraint {
    pub name: String,
    pub id: u32,
    pub penalties: Vec<Penalty>,
    pub priority: ConstraintPriority,
    pub duration: u8,
    pub gap: Option<u8>,
    pub allowed_slots: Option<Vec<(u8, u8)>>,
    pub preferred_slots: Option<Vec<(u8, u8)>>,
}

impl Constraint {
    pub fn calculate_penalty(&self) -> u32 {
        let mut total_penalty = 0;

        for penalty in &self.penalties {
            match penalty {
                Penalty::Validity => total_penalty += 5,
                _ => total_penalty += 3,
            }
        }

        total_penalty
    }
}
