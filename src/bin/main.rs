
extern crate clap;
extern crate sorteador_de_livros_para_ler;

use sorteador_de_livros_para_ler::BookView;

use clap::{Arg, App};
use std::str::FromStr;
fn get_list_book_from_db() -> Vec<BookView> {
    BookView::get_all_books()
}


fn convert_read_model_view (read: bool) -> &'static str{
    if read {
        "lido"
    } else {
        "nao lido"
    }
}

fn show_list_books() {
    let books = get_list_book_from_db();
    for book in books {

        println!("{} {} {}", book.id, book.name, convert_read_model_view(book.read));
    }
}

fn show_random_book() {
    match BookView::random_books(){
        Some(book) => {
            println!("{} {} {}", book.id, book.name, convert_read_model_view(book.read))
        },
        None => {
            println!("Sem livro para sortear cadastre mais livros")
        }

    };

}

fn mark_as_read(id: &str) -> bool {
    let book = BookView {
        name: "".to_string(),
        read: false,
        id: i32::from_str(id).unwrap_or(-1)
    };

    match BookView::mark_as_read(book) {
        1 => true,
        _ => false
    }
}

fn insert_book(name: &str, read: &str) -> bool {
    let book = BookView {
        name: name.to_string(), 
        read: FromStr::from_str(read).unwrap(),
        id: 0i32
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
    if options.is_present("marca_como_lido") {
        let id = options.value_of("marca_como_lido").unwrap_or("");

        match mark_as_read(id) {
            true => println!("livro atualizado com sucesso"),
            false => println!("erro ao  atualizado livro sucesso"),
        }
    }
    if options.is_present("sorterar_livro_para_ler") {
        show_random_book()
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

    let arg_mark_read_in_book = &[
                            Arg::with_name("marca_como_lido")
                            .long("marca_como_lido")
                            .short("m")
                            .value_names(&["id"])
    ];

    let arg_get_random_book = &[
                            Arg::with_name("sorterar_livro_para_ler")
                                .long("sorterar_livro_para_ler")
                                .help("Sortear livro nao lido")
                                .short("s")
    ];
    
    let options = App::new("Sorteador de livros para ler")
                    .version("0.0.1")
                    .author("dial_M_For_Monkey <gabrielsouza849@gmail.com>")
                    .about("cli para sortear qual o proximo livro para ler ")
                    .arg(arg_list_books)
                    .args(arg_insert_book)
                    .args(arg_mark_read_in_book)
                    .args(arg_get_random_book)
                    .get_matches();    
    
    show_result(options);
    
}

