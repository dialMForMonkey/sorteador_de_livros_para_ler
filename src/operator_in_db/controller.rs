extern crate diesel;

use diesel::prelude::*;


mod connection;
mod models;
mod schema;

use self::schema::books::dsl::*;

pub struct Book {
    name: String,
    read: bool
}

impl Book {
    fn is_read (r: Option<i32>) -> bool {
        true
    }

    pub fn random_books() -> Self {
        Book {name: String::from("teste"), read:false}
    }

    pub fn search_book(book: Self) -> Option<Book>{
        None
    }

    pub fn get_all_books() -> Vec<Self> {
        
        let conn = connection::get_connection();
        let result = books.load::<models::Book>(&conn)
                    .map(|item: models::Book| {
                        Book { 
                            name: item.name, 
                            read:  item.read.unwrap(),
                            }
                    })
                    ;
        
        result
    }

    pub fn insert_book(book: Self) -> Option<String>{
        None
    }

    pub fn mark_as_read(book: Self) ->  Option<String> {
        None
    }

}