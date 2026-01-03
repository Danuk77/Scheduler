pub mod constraint_builder;

pub enum ConstraintPriority{
    HIGH,
    LOW
}

#[derive(Clone, Debug)]
pub struct Constraint {
    pub name: String,
    pub id: u32,
}

impl Constraint{
}
