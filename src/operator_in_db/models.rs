#[derive(Queryable)]
pub struct Book {
    pub id: Option<i32>,
    pub name: String,
    pub read: Option<i32>,
}
