use crate::{constraints::Constraint, hill_climber::run_hill_climber};

mod constraints;
mod hill_climber;
mod schedule;

use anyhow::Result;
use constraints::constraint_builder::ConstraintBuilder;

fn main() -> Result<()> {
    // TODO: We may benefit from implementing a data structure to store the constraints in a
    // particular order
    let mut constraints: Vec<Constraint> = Vec::new();

    let mut builder = ConstraintBuilder::new();
    constraints.push(
        builder
            .set_id(1)
            .set_name("a".to_string())
            .set_priority(constraints::ConstraintPriority::High)
            .set_duration(4)
            .set_gap(96)
            .build()?,
    );

    builder = ConstraintBuilder::new();
    constraints.push(
        builder
            .set_id(2)
            .set_name("b".to_string())
            .set_priority(constraints::ConstraintPriority::High)
            .set_duration(4)
            .set_gap(96)
            .build()?,
    );

    builder = ConstraintBuilder::new();
    constraints.push(
        builder
            .set_id(3)
            .set_name("c".to_string())
            .set_priority(constraints::ConstraintPriority::High)
            .set_duration(16)
            .set_gap(48)
            .build()?,
    );

    builder = ConstraintBuilder::new();
    constraints.push(
        builder
            .set_id(4)
            .set_name("d".to_string())
            .set_priority(constraints::ConstraintPriority::High)
            .set_duration(32)
            .set_gap(32)
            .build()?,
    );

    run_hill_climber(&mut constraints, 10000);

    Ok(())
}
