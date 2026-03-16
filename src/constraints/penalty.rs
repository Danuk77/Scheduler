use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Penalty {
    Presence,
    AllowedSlots,
    PreferredSlots,
    Gap,
}
