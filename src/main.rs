pub mod ast;
pub mod dart;
pub mod dart_generator;
pub mod lexer;
pub mod parser;
pub mod rust_generator;
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
    dart: Option<Vec<YamlParams>>,
    rust: Option<Vec<YamlParams>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct YamlParams {
    src: String,
    dest: String,

    #[serde(default = "default_as_true")]
    skip_models: bool,

    #[serde(default = "default_as_true")]
    skip_buffers: bool,

    #[serde(default = "default_as_true")]
    skip_rpc: bool,
}

fn default_as_true() -> bool {
    true
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

        #[clap(long, value_parser)]
        skip_models: bool,

        #[clap(long, value_parser)]
        skip_buffers: bool,

        #[clap(long, value_parser)]
        skip_rpc: bool,
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
    Dart,
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
            skip_models,
            skip_buffers,
            skip_rpc,
        } => generate(input, output, lang, *skip_models, *skip_buffers, *skip_rpc)?,

        Commands::Yaml { path } => {
            let mut path_file = File::open(path)?;
            let mut contents = String::new();
            path_file.read_to_string(&mut contents)?;
            let data = serde_yaml::from_str::<YamlData>(contents.as_str()).unwrap();
            let working_dir = path.parent().unwrap();

            if let Some(rust) = data.rust {
                for item in rust.iter() {
                    let src = working_dir.join(&item.src);
                    let dest = working_dir.join(&item.dest);

                    log::info!("Generate rust buffer: {}", src.display());
                    log::info!("Generate rust dest: {}", dest.display());

                    generate(
                        &src.to_str().unwrap().to_string(),
                        &dest.to_str().unwrap().to_string(),
                        &Lang::Rust,
                        item.skip_models,
                        item.skip_buffers,
                        item.skip_rpc,
                    )?
                }
            }

            if let Some(dart) = data.dart {
                for item in dart.iter() {
                    let src = working_dir.join(&item.src);
                    let dest = working_dir.join(&item.dest);

                    log::info!("Generate dart buffer: {}", src.display());
                    log::info!("Generate dart dest: {}", dest.display());

                    generate(
                        &src.to_str().unwrap().to_string(),
                        &dest.to_str().unwrap().to_string(),
                        &Lang::Dart,
                        item.skip_models,
                        item.skip_buffers,
                        item.skip_rpc,
                    )?
                }
            }
        }
    }

    Ok(())
}

fn generate(
    input: &String,
    output: &String,
    lang: &Lang,
    skip_models: bool,
    skip_buffers: bool,
    skip_rpc: bool,
) -> std::io::Result<()> {
    let mut input_file = File::open(input)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let mut lexer = lexer::Lexer::tokenize(&contents);
    let ast = parser::parse(&mut lexer);

    let data = match lang {
        Lang::Rust => rust_generator::generate(&ast, !skip_models, !skip_buffers, !skip_rpc),
        Lang::Dart => dart_generator::generate(&ast, !skip_models, !skip_buffers, !skip_rpc),
    };

    if output == "-" {
        println!("{}", &data);
    } else {
        let mut output_file = File::create(output)?;
        output_file.write_all(data.as_bytes())?;
    }

    Ok(())
}
