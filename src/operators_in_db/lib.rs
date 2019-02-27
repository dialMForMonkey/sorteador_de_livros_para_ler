#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn get_connection() -> SqliteConnection {
    dotenv().ok();

    let string_connection =  env::var("DATABASE_URL")
        .expect("Erro get DATABASE_URL");

    SqliteConnection::establish(string_connection)
        .expect("Erro get connection")
        
}