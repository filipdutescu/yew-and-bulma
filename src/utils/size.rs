use std::fmt::Display;

#[derive(PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Size::Small => "small",
            Size::Medium => "medium",
            Size::Large => "large",
        };

        write!(f, "{size}")
    }
}
