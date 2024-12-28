use generator::*;
use ir::stringify_ir;

use crate::{
    ast::{self, ASTNode},
    lexer::Literal,
    writer::Writer,
};

pub mod generator;
pub mod ir;

pub fn generate(ast: &[ASTNode]) -> String {
    let mut ir = vec![];

    ir.append(&mut generate_consts(ast));
    ir.append(&mut generate_models(ast));

    if ast::contains_fn_nodes(ast) {
        ir.append(&mut generate_rpc(ast));
    }

    let mut writer = Writer::default();

    writer.writeln("// GENERATED, DO NOT EDIT");
    writer.writeln("@file:Suppress(\"unused\")");
    writer.writeln("");

    let package = ast::find_directive_group_values(ast, "kotlin", "package");

    if package.is_empty() {
        panic!("Kotlin should contain package directive");
    }

    if package.len() > 1 {
        panic!("Kotlin should contain only one package directive");
    }

    let package = match &package[0] {
        ast::ConstValueASTNode::Literal {
            literal,
            type_id: _,
        } => match literal {
            Literal::StringLiteral(value) => value,
            _ => panic!("kotlin import should be a string literal"),
        },
    };
    writer.writeln(&format!("package {}", package));

    let has_rpc = ast::contains_fn_nodes(ast);
    let has_buffers = ast::contains_buffers_nodes(ast);
    let has_consts = ast::contains_consts_nodes(ast);
    let has_signals = ast::contains_signal_fn_nodes(ast);

    if has_rpc || has_buffers || has_consts {
        writer.writeln("");
    }

    if has_buffers || has_rpc {
        writer.writeln("import com.tech_paws.buffers.*");
    }

    if has_signals {
        writer.writeln("import kotlinx.coroutines.flow.Flow");
        writer.writeln("import kotlinx.coroutines.flow.MutableStateFlow");
    }

    if has_rpc || has_buffers {
        writer.writeln("");
    }

    let imports = ast::find_directive_group_values(ast, "kotlin", "import");

    for import in &imports {
        let import = match import {
            ast::ConstValueASTNode::Literal {
                literal,
                type_id: _,
            } => match literal {
                Literal::StringLiteral(value) => value,
                _ => panic!("kotlin import should be a string literal"),
            },
        };
        writer.writeln(&format!("import {}", import));
    }

    if !imports.is_empty() {
        writer.writeln("");
    }

    writer.write(&stringify_ir(&ir));
    writer.show().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::parse};
    use std::fs;

    #[test]
    fn generate_empty_file() {
        let src = fs::read_to_string("test_resources/empty.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/empty.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate(&ast);

        println!("{}", actual);

        assert_eq!(actual.trim(), target.trim());
    }

    #[test]
    fn generate_import_all_file() {
        let src = fs::read_to_string("test_resources/import_all.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/import_all.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate(&ast);

        println!("{}", actual);

        assert_eq!(actual.trim(), target.trim());
    }

    #[test]
    fn generate_import_buffers_file() {
        let src = fs::read_to_string("test_resources/import_buffers.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/import_buffers.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate(&ast);

        println!("{}", actual);

        assert_eq!(actual.trim(), target.trim());
    }

    #[test]
    fn generate_import_fns_file() {
        let src = fs::read_to_string("test_resources/import_fns.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/import_fns.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate(&ast);

        println!("{}", actual);

        assert_eq!(actual.trim(), target.trim());
    }

    #[test]
    fn generate_import_signals_file() {
        let src = fs::read_to_string("test_resources/import_signals.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/import_signals.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate(&ast);

        println!("{}", actual);

        assert_eq!(actual.trim(), target.trim());
    }

    #[test]
    fn generate_import_consts_file() {
        let src = fs::read_to_string("test_resources/import_consts.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/import_consts.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate(&ast);

        println!("{}", actual);

        assert_eq!(actual.trim(), target.trim());
    }

    #[test]
    fn panic_when_more_than_one_package() {
        let src = fs::read_to_string("test_resources/kotlin_two_packages.tpb").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);

        let result = std::panic::catch_unwind(|| generate(&ast));
        assert!(result.is_err());
    }
}
