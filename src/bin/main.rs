
extern crate clap;
extern crate sorteador_de_livros_para_ler;

use sorteador_de_livros_para_ler::BookView;

use clap::{Arg, App};
use std::str::FromStr;
fn get_list_book_from_db() -> Vec<BookView> {
    BookView::get_all_books()
}

fn show_list_books() {
    let books = get_list_book_from_db();
    for book in books {
        println!("{}", book.name)
    }
}

fn insert_book(name: &str, read: &str) -> bool {
    let book = BookView {
        name: name.to_string(), 
        read: FromStr::from_str(read).unwrap()
    };

    match BookView::insert_book(book){
        1 => true,
        _ => false
    }
}

fn show_result(options: clap::ArgMatches) {
    
    if options.is_present("listar_livros") {
        show_list_books();
    }
    if options.is_present("inserir_livro") {
        let name = options.value_of("inserir_livro").unwrap();
        let read = options.value_of("lido").unwrap_or("false");

        match insert_book(name, read) {
            true => println!("cadastrado livro"),
            false => println!("Nao cadastrado livro"),
        }
    }
    
}

fn main() {

    let arg_list_books = Arg::with_name("listar_livros")
                            .short("l")
                            .long("listar_livros")
                            .help("Listar todos os livros cadastrados");

    let arg_insert_book = &[
                        Arg::with_name("inserir_livro")
                            .long("inserir_livro")
                            .short("i")
                            .value_names(&["livro"])
                            .help("Inserir livro"),
                        Arg::with_name("lido")
                            .takes_value(true)
                            .help("marcar como lido")
                        ];
    
    let options = App::new("Sorteador de livros para ler")
                    .version("0.0.1")
                    .author("dial_M_For_Monkey <gabrielsouza849@gmail.com>")
                    .about("cli para sortear qual o proximo livro para ler ")
                    .arg(arg_list_books)
                    .args(arg_insert_book)
                    .get_matches();    
    
    show_result(options);
    
}

