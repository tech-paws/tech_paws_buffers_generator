use crate::{parser::EnumASTNode, writer::Writer};

pub fn generate_enum_into_buffers(node: &EnumASTNode) -> String {
    let mut writer = Writer::new(2);



    writer.show().to_string()
}
