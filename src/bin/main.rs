
extern crate diesel;

extern crate sorteador_de_livros_para_ler;

use sorteador_de_livros_para_ler::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
    use self::schema::books::dsl::*;


    let conn = get_connection();
    
    let results = books.load::<Book>(&conn).expect("Erro Consultar ");
    
    
    println!("Hello, world!");
}
