#[macro_use]

extern crate diesel;

use diesel::prelude::*;
use diesel::insert_into;


mod connection;
mod models;
mod schema;

use self::schema::books::dsl::*;



impl models::BookForm {
    fn is_read (r: Option<i32>) -> bool {
        let result = if let Some(r) = Some(1) {
            true
        } else {
            false
        };
        
        result
    }

    pub fn random_books() -> Self {
        models:: {name: String::from("teste"), read:false, id: 1i32}
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
                            id: 1i32,
                            }
                    })
                    .collect::<Vec<Self>>(),
            Err(_) => vec![]
        }
    }

    pub fn insert_book(book: Self)  {
        

        let conn = connection::get_connection();
        // consultar ultima linha

        let list_book = vec![book];

        insert_into(books).values(list_book).execute(&conn).unwrap();
        


        
    }

    pub fn mark_as_read(book: Self) ->  Option<String> {
        None
    }

}