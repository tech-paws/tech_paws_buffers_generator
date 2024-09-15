use ir::stringify_ir;

use crate::{
    ast::ASTNode,
    writer::Writer,
};

pub mod generator;
pub mod ir;

pub fn generate(_ast: &[ASTNode]) -> String {
    let ir = vec![];

    let mut writer = Writer::new(2);

    writer.writeln("// GENERATED, DO NOT EDIT");
    writer.writeln("");
    writer.write(&stringify_ir(&ir));
    writer.show().to_string()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
