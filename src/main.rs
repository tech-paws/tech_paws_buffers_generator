pub mod ast;
pub mod kotlin;
pub mod lexer;
pub mod parser;
// pub mod dart;
pub mod rust;
pub mod rust_generator;
pub mod swift;
pub mod writer;

use std::{
    env,
    fs::File,
    io::{Read, Write},
    path,
};

use clap::{clap_derive::ArgEnum, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Serialize, Deserialize, Debug)]
struct YamlData {
    rust: Option<Vec<YamlParams>>,
    swift: Option<Vec<YamlParams>>,
    kotlin: Option<Vec<YamlParams>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct YamlParams {
    src: String,
    dest: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate {
        #[clap(short, long, value_parser)]
        input: String,

        #[clap(short, long, value_parser)]
        output: String,

        #[clap(short, long, arg_enum)]
        lang: Lang,
    },
    Yaml {
        /// Path to a config
        #[clap(value_parser)]
        path: path::PathBuf,
    },
}

#[derive(ArgEnum, Clone, Debug)]
enum Lang {
    Rust,
    Swift,
    Kotlin,
}

fn main() -> std::io::Result<()> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .parse_env(&env::var("TPBUFFER_LOG").unwrap_or_default())
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate {
            input,
            output,
            lang,
        } => generate(input, output, lang)?,

        Commands::Yaml { path } => {
            let mut path_file = File::open(path)?;
            let mut contents = String::new();
            path_file.read_to_string(&mut contents)?;
            let data = serde_yaml::from_str::<YamlData>(contents.as_str()).unwrap();
            let working_dir = path.parent().unwrap();

            if let Some(rust) = &data.rust {
                for item in rust.iter() {
                    let src = working_dir.join(&item.src);
                }
            }

            if let Some(rust) = &data.rust {
                for item in rust.iter() {
                    let src = working_dir.join(&item.src);
                    let dest = working_dir.join(&item.dest);

                    log::info!("Generate rust buffer: {}", src.display());
                    log::info!("Generate rust dest: {}", dest.display());

                    generate(
                        &src.to_str().unwrap().to_string(),
                        &dest.to_str().unwrap().to_string(),
                        &Lang::Rust,
                    )?
                }
            }

            if let Some(swift) = data.swift {
                for item in swift.iter() {
                    let src = working_dir.join(&item.src);
                    let dest = working_dir.join(&item.dest);

                    log::info!("Generate swift buffer: {}", src.display());
                    log::info!("Generate swift dest: {}", dest.display());

                    generate(
                        &src.to_str().unwrap().to_string(),
                        &dest.to_str().unwrap().to_string(),
                        &Lang::Swift,
                    )?
                }
            }

            if let Some(kotlin) = data.kotlin {
                for item in kotlin.iter() {
                    let src = working_dir.join(&item.src);
                    let dest = working_dir.join(&item.dest);

                    log::info!("Generate kotlin buffer: {}", src.display());
                    log::info!("Generate kotlin dest: {}", dest.display());

                    generate(
                        &src.to_str().unwrap().to_string(),
                        &dest.to_str().unwrap().to_string(),
                        &Lang::Kotlin,
                    )?
                }
            }
        }
    }

    Ok(())
}

fn generate(input: &String, output: &String, lang: &Lang) -> std::io::Result<()> {
    let mut input_file = File::open(input)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let mut lexer = lexer::Lexer::tokenize(&contents);
    let ast = parser::parse(&mut lexer);

    let data: String = match lang {
        Lang::Rust => rust_generator::generate(&ast),
        Lang::Swift => swift::generate(&ast),
        Lang::Kotlin => "Not Implemented".to_string(),
    };

    if output == "-" {
        println!("{}", &data);
    } else {
        let mut output_file = File::create(output)?;
        output_file.write_all(data.as_bytes())?;
    }

    Ok(())
}
