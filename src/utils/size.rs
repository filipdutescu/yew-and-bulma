use std::fmt::Display;

#[derive(PartialEq)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Size::Small => "small",
            Size::Normal => "normal",
            Size::Medium => "medium",
            Size::Large => "large",
        };

        write!(f, "{size}")
    }
}
