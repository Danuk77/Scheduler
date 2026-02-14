use crate::schedule::Slot;

pub enum ChangeType {
    Move(Slot, Slot),     // (New slot, previous slot)
    Scheduled(u32),       // Constraint id
    Subtituted(u32, u32), // (New constraint, previous constraint)
    // TODO: This is not implemented yet
    //Swap(Slot, Slot)
}
