use std::array;

pub struct Schedule {
    pub grid: [[Option<u32>; 48]; 7], // Option<u32> stores the id of the constraint, or None if
                                      // nothing is scheduled
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            grid: array::from_fn(|_| array::from_fn(|_| None)),
        }
    }
}
