use crate::{constraints::constraint_store::ConstraintStore, hill_climber::run_hill_climber};

mod constraints;
mod hill_climber;
mod schedule;

use anyhow::Result;
use constraints::constraint_builder::ConstraintBuilder;

fn main() -> Result<()> {
    println!("Initialising constraint store");
    let mut constraints: ConstraintStore = ConstraintStore::new();

    let mut builder = ConstraintBuilder::new();
    constraints.push(
        builder
            .set_id(1)
            .set_name("a".to_string())
            .set_priority(constraints::ConstraintPriority::High)
            .set_duration(4)
            .build()?,
    );

    builder = ConstraintBuilder::new();
    constraints.push(
        builder
            .set_id(2)
            .set_name("b".to_string())
            .set_priority(constraints::ConstraintPriority::High)
            .set_duration(4)
            .build()?,
    );

    builder = ConstraintBuilder::new();
    constraints.push(
        builder
            .set_id(3)
            .set_name("c".to_string())
            .set_priority(constraints::ConstraintPriority::High)
            .set_duration(16)
            .build()?,
    );

    builder = ConstraintBuilder::new();
    constraints.push(
        builder
            .set_id(4)
            .set_name("d".to_string())
            .set_priority(constraints::ConstraintPriority::High)
            .set_duration(32)
            .build()?,
    );

    println!("Running hill climber algorithm");
    run_hill_climber(&mut constraints, 1);
    println!("Finished hill climber");

    Ok(())
}
