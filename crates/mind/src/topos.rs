use std::fmt;

pub enum Topos {
    Nil,
}

impl fmt::Debug for Topos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Topos::Nil => { write!(f, "Nil") }
        }
    }
}

impl Clone for Topos {
    fn clone(&self) -> Self {
        match self {
            Self::Nil => Self::Nil,
        }
    }
}