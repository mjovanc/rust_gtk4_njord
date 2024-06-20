use njord::table::Table;
use njord_derive::Table;

#[derive(Table, Clone)]
#[table_name = "tasks"]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub status: String,
}
