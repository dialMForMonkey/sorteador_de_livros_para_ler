#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub read: bool,
}
