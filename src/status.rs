#[derive(Debug, PartialEq)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    pub fn to_uppercase_string(&self) -> String {
        match self {
            Status::Todo => String::from("TODO"),
            Status::InProgress => String::from("IN PROGRESS"),
            Status::Done => String::from("DONE"),
        }
    }
}
