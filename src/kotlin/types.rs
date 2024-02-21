use crate::ast::TypeIDASTNode;

pub fn generate_type_id(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer { size, signed, .. } => match size {
            1 if *signed => String::from("Byte"),
            4 if *signed => String::from("Int"),
            8 if *signed => String::from("Long"),
            1 if !*signed => String::from("UByte"),
            4 if !*signed => String::from("UInt"),
            8 if !*signed => String::from("ULong"),
            _ => panic!("Unsupported integer size, {}", size),
        },
        TypeIDASTNode::Number { size, .. } => match size {
            4 => String::from("Float"),
            8 => String::from("Double"),
            _ => panic!("Unsupported number size, {}", size),
        },
        TypeIDASTNode::Bool { .. } => String::from("Boolean"),
        TypeIDASTNode::Char { id } => id.clone(),
        TypeIDASTNode::Other { id } => id.clone(),
        TypeIDASTNode::Generic { id, generics } => {
            format!(
                "{}<{}>",
                id,
                generics
                    .iter()
                    .map(generate_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::TypeIDASTNode, kotlin::types::generate_type_id};

    #[test]
    fn generate_const_value_test_signed_integers() {
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("i8"),
                size: 1,
                signed: true,
            }),
            String::from("Byte")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("i32"),
                size: 4,
                signed: true,
            }),
            String::from("Int")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("i64"),
                size: 8,
                signed: true,
            }),
            String::from("Long")
        );
    }

    #[test]
    fn generate_const_value_test_unsigned_integers() {
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("u8"),
                size: 1,
                signed: false,
            }),
            String::from("UByte")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("u32"),
                size: 4,
                signed: false,
            }),
            String::from("UInt")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("u64"),
                size: 8,
                signed: false,
            }),
            String::from("ULong")
        );
    }

    #[test]
    fn generate_const_value_test_numbers() {
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Number {
                id: String::from("f32"),
                size: 4,
            }),
            String::from("Float")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Number {
                id: String::from("f64"),
                size: 8,
            }),
            String::from("Double")
        );
    }

    #[test]
    fn generate_const_value_test_boolean() {
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Bool {
                id: String::from("bool")
            }),
            String::from("Boolean")
        );
    }
}
