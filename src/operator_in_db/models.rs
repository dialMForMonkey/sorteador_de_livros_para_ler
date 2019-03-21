use schema::books;

#[derive(Queryable)]
pub struct Book {
    pub id: Option<i32>,
    pub name: String,
    pub read: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct BookForm {
   pub name: String,
   pub read: i32
}