use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ScheduleError {
    ConstraintNotScheduled(u32),
}

impl fmt::Display for ScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScheduleError::ConstraintNotScheduled(constraint_id) => {
                write!(f, "Constraint '{}' is not scheduled", constraint_id)
            }
        }
    }
}

impl Error for ScheduleError {}
