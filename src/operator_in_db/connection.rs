
extern crate diesel;
extern crate dotenv;


use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use self::dotenv::dotenv;

use std::env;

pub fn get_connection() -> SqliteConnection {
    dotenv().ok();

    let string_connection = match env::var("DATABASE_URL") {
        Ok(x) => x,
        Err(err) => {
            println!("{}", err.to_string());
           String::from("")
        }
    };

    SqliteConnection::establish(&string_connection)
        .expect("Erro get connection")
        
}
