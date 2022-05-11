use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Struct,
    ID { name: String },
    StringLiteral { value: String },
    IntLiteral { value: i64 },
    NumberLiteral { value: f64 },
    Symbol { value: char },
    EOF,
}

pub struct Lexer {
    cursor: usize,
    tokens: Vec<Token>,
}

struct StringReader<'a> {
    data: &'a str,
    cursor: usize,
}

impl<'a> StringReader<'a> {
    fn new(data: &'a str) -> Self {
        Self { data, cursor: 0 }
    }

    pub fn next(&mut self) -> Option<char> {
        self.cursor += 1;
        self.current()
    }

    pub fn current(&self) -> Option<char> {
        if self.cursor < self.data.len() {
            Some(self.data.as_bytes()[self.cursor] as char)
        } else {
            None
        }
    }
}

impl Lexer {
    pub fn tokenize(data: &str) -> Lexer {
        let mut tokens = Vec::with_capacity(100);
        let mut string_reader = StringReader::new(data);

        loop {
            if let Some(ch) = string_reader.current() {
                if is_char_id(ch, true) {
                    let token = lex_id(&mut string_reader);
                    tokens.push(token);
                } else if ch == '"' {
                    let token = lex_string(&mut string_reader);
                    tokens.push(token);
                } else if is_char_number(ch) {
                    let token = lex_number(&mut string_reader);
                    tokens.push(token);
                } else if ch == ' ' {
                    string_reader.next();
                    continue;
                } else {
                    tokens.push(Token::Symbol { value: ch });
                }

                string_reader.next();
            } else {
                break;
            }
        }

        tokens.push(Token::EOF);
        Lexer { tokens, cursor: 0 }
    }

    pub fn next_token(&mut self) -> &Token {
        self.cursor += 1;
        self.current_token()
    }

    pub fn current_token(&self) -> &Token {
        if self.cursor >= self.tokens.len() {
            self.tokens.last().unwrap()
        } else {
            &self.tokens[self.cursor]
        }
    }
}

fn is_char_number(ch: char) -> bool {
    let numbers = "0123456789";
    numbers.contains(ch)
}

fn is_char_id(ch: char, first_char: bool) -> bool {
    let letters = "abcdefjhigklmnopqrstuvwxyzABCDEFJHIGKLMNOPQRSTUVWXYZ_";
    let numbers = "0123456789";

    if first_char {
        letters.contains(ch)
    } else {
        letters.contains(ch) || numbers.contains(ch)
    }
}

fn lex_id(string_reader: &mut StringReader) -> Token {
    let mut name = String::new();

    loop {
        name += &String::from(string_reader.current().unwrap());
        let next = string_reader.next();

        match next {
            Some(ch) => {
                if !is_char_id(ch, false) {
                    break;
                }
            }
            None => break,
        }
    }

    match name.as_str() {
        "struct" => Token::Struct,
        _ => Token::ID { name },
    }
}

fn lex_string(string_reader: &mut StringReader) -> Token {
    let mut value = String::new();

    if string_reader.next().is_none() {
        panic!("TODO: Use lex error")
    }

    loop {
        let current = string_reader.current().unwrap();

        if current == '"' {
            break;
        }

        value += &String::from(current);
        let next = string_reader.next();

        if let Some(_) = next {
            continue;
        }
    }

    Token::StringLiteral { value }
}

struct StateMachine {}

enum StateOutput {
    Transition { state: &'static str },
    Finish { name: &'static str },
    Error { message: &'static str },
}

// State machine
//
//                                                +------------------+
//                                 +---- other -> | Error            |
//                                 |              +------------------+     +------------------+
//                                 |                   +---- other ------->| Int literal (16) |
// +---+          +---+          +---+               +---+                 +------------------+
// | I | - '0' -> | 1 | - 'x' -> | 2 | - '0'..'9' -> | 3 | - '0'..'9' -+
// +---+          +---+          +---+               +---+             |
//   |              |                                  ^               |
//   |              |                                  +---------------+
//   |              |                                                 +------------------+
//   +--------------+                   +---- other ------------------| Int literal (10) |
//                  |                   |                             +------------------+
//                  |                   |                             +----------------+
//                  |                   |              +---- other -> | Number literal |
//                  |                   |              |              +----------------+
//                  |                   |            +---+
//                  |                   +---- '.' -> | 5 | - '0'..'9' -+
//                  |                   |            +---+             |
//                  |                   |              ^               |
//                  |                   |              +---------------+
//                  |                 +---+
//                  +---- '0'..'9' -> | 4 | - '0'..'9' -+
//                                    +---+             |
//                                      ^               |
//                                      +---------------+
//
fn lex_number(string_reader: &mut StringReader) -> Token {
    let mut value = String::new();
    let mut current_state = 0;
    let current = string_reader.current().unwrap();

    let state_machine = StateMachine::new();

    state_machine.add_state('I', |ch| {
        if ch == '0' {
            StateOutput::Transition { state: "1" }
        } else if is_char_number(ch) {
            StateOutput::Transition { state: "4" }
        } else {
            StateOutput::Finish {
                name: "Int literal (10)",
            }
        }
    });

    state_machine.add_state('0', |ch| {
        if ch == '0' {
            StateOutput::Transition { state: "1" }
        } else if is_char_number(ch) {
            StateOutput::Transition { state: "4" }
        } else {
            StateOutput::Finish {
                name: "Int literal (10)",
            }
        }
    });

    // let init_state: |char| -> ();

    // init_state = Rc::new();

    // current_state = init_state.clone();

    // loop {
    //     value += &String::from(string_reader.current().unwrap());
    //     let next = string_reader.next();

    //     match next {
    //         Some(ch) => {
    //             if !is_char_number(ch) {
    //                 break;
    //             }
    //         }
    //         None => break,
    //     }
    // }

    Token::IntLiteral {
        // TODO(sysint64): handle error
        value: i64::from_str_radix(&value, 10).unwrap(),
    }
}

mod test {
    use super::*;

    #[test]
    fn string_reader_simple() {
        let mut string_reader = StringReader::new("a");
        assert_eq!(string_reader.current(), Some('a'));
        assert_eq!(string_reader.next(), None);
    }

    #[test]
    fn string_reader_abc123() {
        let mut string_reader = StringReader::new("abc123");
        assert_eq!(string_reader.current(), Some('a'));
        assert_eq!(string_reader.next(), Some('b'));
        assert_eq!(string_reader.next(), Some('c'));
        assert_eq!(string_reader.next(), Some('1'));
        assert_eq!(string_reader.next(), Some('2'));
        assert_eq!(string_reader.next(), Some('3'));
        assert_eq!(string_reader.current(), Some('3'));
        assert_eq!(string_reader.next(), None);
        assert_eq!(string_reader.current(), None);
    }

    #[test]
    fn lex_empty() {
        let mut lexer = Lexer::tokenize("");
        let token = lexer.current_token();
        assert_eq!(token.clone(), Token::EOF);
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    fn lex_id() {
        let mut lexer = Lexer::tokenize("hello     world123");
        let token = lexer.current_token();
        assert_eq!(
            token.clone(),
            Token::ID {
                name: String::from("hello")
            }
        );
        let token = lexer.next_token();
        assert_eq!(
            token.clone(),
            Token::ID {
                name: String::from("world123")
            }
        );
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    fn lex_string_literal() {
        let mut lexer = Lexer::tokenize("\"hello     world\"");
        let token = lexer.current_token();
        assert_eq!(
            token.clone(),
            Token::StringLiteral {
                value: String::from("hello     world")
            }
        );
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    fn lex_int_literal() {
        let mut lexer = Lexer::tokenize("123");
        let token = lexer.current_token();
        assert_eq!(token.clone(), Token::IntLiteral { value: 123 });
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    #[ignore]
    fn lex_number_literal() {
        let mut lexer = Lexer::tokenize("123. 342.23 03.001");
        let token = lexer.current_token();
        assert_eq!(token.clone(), Token::NumberLiteral { value: 123. });
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::NumberLiteral { value: 324.23 });
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::NumberLiteral { value: 3.001 });
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    fn lex_struct_keyword() {
        let mut lexer = Lexer::tokenize("struct");
        let token = lexer.current_token();
        assert_eq!(token.clone(), Token::Struct);
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }
}
