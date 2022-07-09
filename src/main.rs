pub mod ast;
pub mod dart;
pub mod dart_generator;
pub mod lexer;
pub mod parser;
pub mod rust_generator;
pub mod writer;

use std::{
    fs::File,
    io::{Read, Write},
};

use clap::{clap_derive::ArgEnum, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser)]
    input: String,

    #[clap(short, long, value_parser)]
    output: String,

    #[clap(short, long, arg_enum)]
    lang: Lang,

    #[clap(long, value_parser)]
    skip_models: bool,

    #[clap(long, value_parser)]
    skip_buffers: bool,

    #[clap(long, value_parser)]
    skip_rpc: bool,
}

#[derive(ArgEnum, Clone, Debug)]
enum Lang {
    Rust,
    Dart,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let mut input_file = File::open(cli.input)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let mut lexer = lexer::Lexer::tokenize(&contents);
    let ast = parser::parse(&mut lexer);

    let data = match cli.lang {
        Lang::Rust => {
            rust_generator::generate(&ast, !cli.skip_models, !cli.skip_buffers, !cli.skip_rpc)
        }
        Lang::Dart => {
            dart_generator::generate(&ast, !cli.skip_models, !cli.skip_buffers, !cli.skip_rpc)
        }
    };

    if cli.output == "-" {
        println!("{}", &data);
    } else {
        let mut output_file = File::create(cli.output)?;
        output_file.write_all(data.as_bytes())?;
    }

    Ok(())
}
