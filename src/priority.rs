#[derive(Debug, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn to_uppercase_string(&self) -> String {
        match self {
            Priority::Low => String::from("LOW"),
            Priority::Medium => String::from("MEDIUM"),
            Priority::High => String::from("HIGH"),
        }
    }
}
