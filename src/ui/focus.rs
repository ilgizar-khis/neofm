#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Focus {
    Side,
    List,
    View,
}

impl Focus {
    pub fn next(&self) -> Self {
        match self {
            Self::Side => Self::List,
            Self::List => Self::View,
            Self::View => Self::Side,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Side => Self::View,
            Self::List => Self::Side,
            Self::View => Self::List,
        }
    }
}
