use std::array;

use crate::constraints::Constraint;

pub struct Schedule {
    pub grid: [[Option<Constraint>; 48]; 7],
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            grid: array::from_fn(|_| array::from_fn(|_| None)),
        }
    }
}
