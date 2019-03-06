
extern crate clap;
extern crate sorteador_de_livros_para_ler;

use sorteador_de_livros_para_ler::Book;

use clap::{Arg, App};
fn get_list_book_from_db() -> Vec<Book> {
    Book::get_all_books()
}

fn show_list_books() {
    let books = get_list_book_from_db();
    for book in books {
        println!("{}", book.name)
    }
}

fn main() {

    let arg_list_books = Arg::with_name("listar_livros")
                            .short("l")
                            .long("listar_livros")
                            .help("Listar todos os livros cadastrados");
    
    let options = App::new("Sorteador de livros para ler")
                    .version("0.0.1")
                    .author("dial_M_For_Monkey <gabrielsouza849@gmail.com>")
                    .about("cli para sortear qual o proximo livro para ler ")
                    .arg(arg_list_books)
                    .get_matches();    
    
    show_list_books()
}

