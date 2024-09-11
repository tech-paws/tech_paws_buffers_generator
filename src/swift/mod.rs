use crate::{
    ast::{self, ASTNode},
    lexer::Literal,
    writer::Writer,
};

use self::{
    generator::{generate_consts, generate_models, generate_rpc},
    ir::stringify_ir,
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
    writer.writeln("");
    writer.writeln("import Foundation");
    writer.writeln("import Combine");
    writer.writeln("");

    let imports = ast::find_directive_group_values(ast, "swift", "import");

    for import in imports {
        let import = match import {
            ast::ConstValueASTNode::Literal {
                literal,
                type_id: _,
            } => match literal {
                Literal::StringLiteral(value) => value,
                _ => panic!("swift import should be a string literal"),
            },
        };
        writer.writeln(&format!("import {}", import));
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
        let target = fs::read_to_string("test_resources/swift/empty.swift").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate(&ast);
        assert_eq!(actual.trim(), target.trim());
    }
}
