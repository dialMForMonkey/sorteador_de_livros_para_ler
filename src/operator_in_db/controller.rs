#[macro_use]

extern crate diesel;

use diesel::prelude::*;


mod connection;
mod models;
mod schema;

use self::schema::books::dsl::*;

pub struct Book {
   pub name: String,
   pub read: bool
}

impl Book {
    fn is_read (r: Option<i32>) -> bool {
        let result = if let Some(r) = Some(1) {
            true
        } else {
            false
        };
        
        result
    }

    pub fn random_books() -> Self {
        Book {name: String::from("teste"), read:false}
    }

    pub fn search_book(book: Self) -> Option<Book>{
        None
    }

     pub fn get_all_books()  -> Vec<Self> {
        
        let conn = connection::get_connection();
        match books.load::<models::Book>(&conn) {
            Ok(result) => result
                    .into_iter()
                    .map(|item: models::Book| {
                        Book { 
                            name: item.name, 
                            read: Self::is_read(item.read),
                            }
                    })
                    .collect::<Vec<Self>>(),
            Err(_) => vec![]
        }
    }

    pub fn insert_book(book: Self) -> Option<String>{
        None
    }

    pub fn mark_as_read(book: Self) ->  Option<String> {
        None
    }

}