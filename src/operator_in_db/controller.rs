#[macro_use]

extern crate diesel;
extern crate rand;

use rand::prelude::*;
use diesel::prelude::*;
use diesel::insert_into;

use diesel::update;

mod connection;
mod models;
mod schema;

use self::schema::books::dsl::*;


pub struct BookView {
   pub name: String,
   pub read: bool,
   pub id: i32,
}

impl BookView  {
    fn is_read (_r: Option<i32>) -> bool {
        match _r {
            Some(i) => {
                if i == 1 {
                    true
                } else {
                    false
                }
            },
            None => false
        }
    }

    fn convert_view_for_model(_r: bool) -> i32 {
         if let true = _r {
                1
            } else {
                0
            }
    }

    fn convert_list_model_for_view(_r: Vec<models::Book>) -> Vec<Self> {
        _r.into_iter()
            .map(|item: models::Book| {
                BookView { 
                    name: item.name,
                    read: Self::is_read(item.read),
                    id: item.id.unwrap_or(0)
                    }
            })
            .collect::<Vec<Self>>()
    }

    pub fn random_books() -> Option<BookView> {
        let all_books_ready_read =  Self::get_all_books().into_iter().filter(|item| !item.read).collect::<Vec<BookView>>();
        let mut rng = rand::thread_rng();
        let max_number = all_books_ready_read.len();
        if max_number > 0 {
            let sort_number = rng.gen_range(0,max_number);

            all_books_ready_read.into_iter().nth(sort_number)
        } else {
            None
        }
    }

    pub fn search_book(_book: Self) -> Vec<Self> {
       let conn = connection::get_connection();
       let result = books
        .filter(name.like(_book.name))
        .load(&conn);

        match result {
            Ok(result) => Self::convert_list_model_for_view(result),
            Err(_) => vec![]
        }
    }

     pub fn get_all_books()  -> Vec<Self> {
        
        let conn = connection::get_connection();
        match books.load::<models::Book>(&conn) {
            Ok(result) => Self::convert_list_model_for_view(result),
            Err(_) => vec![]
        }
    }

    pub fn insert_book(_book_view: Self) -> usize {
        
        let conn = connection::get_connection();
        let book_model =  models::BookForm {
            name: _book_view.name,
            read: Self::convert_view_for_model(_book_view.read),
        };
        let list_book = vec![book_model];

        insert_into(books)
            .values(list_book)
            .execute(&conn)
            .unwrap_or(0)
    }

    pub fn mark_as_read(_book: Self) -> usize {
        let conn = connection::get_connection();

        update(books.filter(id.eq(_book.id)))
            .set(
                read.eq(
                        match _book.read {
                            true => 1,
                            false => 0,
                        }
                    )
            )
            .execute(&conn)
            .unwrap_or_else(|er|{
                println!("{:?}",er);
                0
            })
    }

}