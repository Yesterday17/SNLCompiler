use clap::{App, Arg};
use std::io::Read;

fn main() {
    let matches = App::new("SNL Compiler")
        .arg(Arg::with_name("filename")
            .required(true)
            .takes_value(true)
            .empty_values(false)
            .multiple(false)
        )
        .get_matches();

    let file = matches.value_of("filename").expect("Filename not provided");
    let input = if file == "-" {
        unimplemented!()
    } else {
        let mut file = std::fs::File::open(file).expect("Failed to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Failed to read file");
        data
    };
    let lex_result = snl_lexer::read_tokens(&input).unwrap();
    for token in lex_result {
        println!("{}", token);
    }
}
