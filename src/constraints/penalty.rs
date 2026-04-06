use core::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Penalty {
    Presence,
    AllowedSlots,
    PreferredSlots,
    Gap,
}

impl Display for Penalty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Presence => write!(f, "Presence"),
            Self::AllowedSlots => write!(f, "Allowed slots"),
            Self::Gap => write!(f, "Gap"),
            Self::PreferredSlots => write!(f, "PreferredSlots"),
        }
    }
}
