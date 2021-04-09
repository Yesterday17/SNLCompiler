use clap::{App, Arg};
use std::io::Read;
use std::process::exit;
use snl_semantic::Semantic;

fn main() {
    let matches = App::new("SNL Compiler")
        .arg(Arg::with_name("mode")
            .long("mode")
            .required(true)
            .takes_value(true)
            .possible_values(&["lex", "parse", "semantic"])
            .default_value("semantic")
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
            .possible_values(&["rdp", "ll1"])
            .default_value("rdp")
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
        _ => unreachable!(),
    };
    if mode == "lex" {
        for token in tokens {
            println!("{}", token);
        }
        exit(0);
    }

    let ast = match matches.value_of("parser") {
        Some("rdp") => {
            let parser = snl_rdp::Parser::new(tokens);
            parser.parse().expect("Failed to parse")
        }
        Some("ll1") => {
            let mut parser = snl_ll1::Parser::new(tokens);
            parser.parse().expect("Failed to parse")
        }
        None => panic!("no parser specified"),
        _ => unreachable!(),
    };
    if mode == "parse" {
        println!("{}", serde_json::to_string(&ast).unwrap());
        exit(0);
    }

    assert_eq!(mode, "semantic");
    let errors = Semantic::new(ast).analyze();
    if errors.is_empty() {
        println!("No semantic error!");
    } else {
        for error in errors {
            println!("At line {}, column {}:\t{}", error.line, error.column, error.inner());
        }
    }
}
