#[derive(Table)]
#[table_name = "tasks"]
pub struct Task {
    id: u32,
    title: String,
    description: String,
    priority: String,
    status: String,
}
