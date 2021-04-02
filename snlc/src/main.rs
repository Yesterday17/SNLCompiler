use clap::{App, Arg};
use std::io::Read;
use std::process::exit;

fn main() {
    let matches = App::new("SNL Compiler")
        .arg(Arg::with_name("mode")
            .long("mode")
            .required(true)
            .takes_value(true)
            .possible_values(&["lex", "parse", "semantic"])
            .default_value("parse")
        )
        .arg(Arg::with_name("lexer")
            .long("lexer")
            .short("l")
            .required(true)
            .takes_value(true)
            .possible_values(&["rs", "c"])
            .default_value("rs")
        )
        .arg(Arg::with_name("parser")
            .long("parser")
            .short("p")
            .required(false)
            .takes_value(true)
            // Recursive descent parser or LL(1) parser
            .possible_values(&["rdp-rs", "ll1-rs", "rdp-c", "ll1-c"])
            .default_value("rdp-rs")
        )
        .arg(Arg::with_name("filename")
            .required(true)
            .takes_value(true)
            .empty_values(false)
            .multiple(false)
        )
        .get_matches();

    let mode = matches.value_of("mode").unwrap();
    let file = matches.value_of("filename").unwrap();
    let input = if file == "-" {
        let mut data = String::new();
        std::io::stdin().read_to_string(&mut data).expect("Failed to read string from stdin");
        data
    } else {
        let mut file = std::fs::File::open(file).expect("Failed to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Failed to read file");
        data
    };

    let tokens = match matches.value_of("lexer").unwrap() {
        "rs" => {
            snl_lexer::read_tokens(&input).unwrap()
        }
        "c" => {
            unimplemented!()
        }
        &_ => unreachable!(),
    };
    if mode == "lex" {
        for token in tokens {
            println!("{}", token);
        }
        exit(0);
    }

    let ast = match matches.value_of("parser") {
        Some("rdp-rs") => {
            let parser = snl_rdp::Parser::new(tokens);
            parser.parse().expect("Failed to parse")
        }
        Some(_) => unimplemented!(),
        None => panic!("no parser specified")
    };
    if mode == "parse" {
        println!("{:#?}", ast);
    }
}
