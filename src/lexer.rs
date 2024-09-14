#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithLineAndPos {
    line: usize,
    pos: usize,
    token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Struct,
    Enum,
    Fn,
    Signal,
    Async,
    Const,
    ID { name: String },
    Literal(Literal),
    Symbol(char),
    DocComment { value: String },
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    StringLiteral(String),
    IntLiteral(i64),
    NumberLiteral(f64),
    BoolLiteral(bool),
}

pub struct Lexer {
    cursor: usize,
    tokens: Vec<TokenWithLineAndPos>,
    registered_fn_positions: Vec<u32>,
    last_fn_position: u32,
}

struct StringReader<'a> {
    data: &'a str,
    cursor: usize,
    line: usize,
    pos: usize,
}

impl<'a> StringReader<'a> {
    fn new(data: &'a str) -> Self {
        Self {
            data,
            cursor: 0,
            line: 1,
            pos: 0,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn next(&mut self) -> Option<char> {
        if self.current() != Option::Some('\n') {
            self.pos += 1;
        }

        self.cursor += 1;

        if self.current() == Option::Some('\n') {
            self.line += 1;
            self.pos = 0;
        }

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

        while let Some(ch) = string_reader.current() {
            if is_char_id(ch, true) {
                let token = lex_id(&mut string_reader);
                tokens.push(token);
            } else if ch == '"' {
                let token = lex_string(&mut string_reader);
                tokens.push(token);
            } else if is_char_number(ch) {
                let token = lex_number(&mut string_reader);
                tokens.push(token);
            } else if ch == ' ' || ch == '\n' || ch == '\r' {
                string_reader.next();
                continue;
            } else if ch == '/' {
                let first_ch_line = string_reader.line();
                let first_ch_pos = string_reader.pos();

                match string_reader.next() {
                    Some('/') => {
                        let ch = string_reader.next();

                        if ch == Some('/') {
                            let token = lex_doc_comment(&mut string_reader);
                            tokens.push(token);
                        } else {
                            lex_single_line_comment(&mut string_reader);
                        }
                    }
                    Some('*') => lex_multi_line_comment(&mut string_reader),
                    _ => {
                        tokens.push(TokenWithLineAndPos {
                            line: first_ch_line,
                            pos: first_ch_pos,
                            token: Token::Symbol('/'),
                        });
                        tokens.push(TokenWithLineAndPos {
                            line: string_reader.line(),
                            pos: string_reader.pos(),
                            token: Token::Symbol(ch),
                        });

                        string_reader.next();
                    }
                }
            } else {
                tokens.push(TokenWithLineAndPos {
                    line: string_reader.line(),
                    pos: string_reader.pos(),
                    token: Token::Symbol(ch),
                });

                string_reader.next();
            }
        }

        tokens.push(TokenWithLineAndPos {
            line: string_reader.line(),
            pos: string_reader.pos(),
            token: Token::EOF,
        });

        Lexer {
            tokens,
            cursor: 0,
            registered_fn_positions: vec![],
            last_fn_position: 0,
        }
    }

    pub fn next_fn_poisition(&mut self) -> u32 {
        let position = self.last_fn_position;

        self.last_fn_position += 1;

        while self
            .registered_fn_positions
            .contains(&self.last_fn_position)
        {
            self.last_fn_position += 1;
        }

        position
    }

    pub fn next_token(&mut self) -> &Token {
        self.cursor += 1;
        self.current_token()
    }

    pub fn rewind_one_token(&mut self) {
        self.cursor -= 1;
    }

    pub fn current_token(&self) -> &Token {
        &self.current_token_with_line_and_pos().token
    }

    fn current_token_with_line_and_pos(&self) -> &TokenWithLineAndPos {
        if self.cursor >= self.tokens.len() {
            self.tokens.last().unwrap()
        } else {
            &self.tokens[self.cursor]
        }
    }

    pub fn line(&self) -> usize {
        self.current_token_with_line_and_pos().line
    }

    pub fn pos(&self) -> usize {
        self.current_token_with_line_and_pos().pos
    }
}

fn is_char_number(ch: char) -> bool {
    let numbers = "0123456789";
    numbers.contains(ch)
}

fn is_char_hex_number(ch: char) -> bool {
    let numbers = "0123456789ABCDEF";
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

fn lex_single_line_comment(string_reader: &mut StringReader) {
    while let Some(ch) = string_reader.current() {
        if ch == '\r' || ch == '\n' {
            string_reader.next();
            return;
        }

        string_reader.next();
    }
}

fn lex_doc_comment(string_reader: &mut StringReader) -> TokenWithLineAndPos {
    string_reader.next();

    let mut comment = String::new();
    let line = string_reader.line();
    let pos = string_reader.pos();

    while let Some(ch) = string_reader.current() {
        if ch == '\r' || ch == '\n' {
            string_reader.next();
            break;
        }

        comment.push(ch);
        string_reader.next();
    }

    TokenWithLineAndPos {
        line,
        pos,
        token: Token::DocComment { value: comment },
    }
}

fn lex_multi_line_comment(string_reader: &mut StringReader) {
    while let Some(ch) = string_reader.current() {
        if ch == '*' {
            if let Some('/') = string_reader.next() {
                string_reader.next();
                return;
            }
        }

        string_reader.next();
    }
}

fn lex_id(string_reader: &mut StringReader) -> TokenWithLineAndPos {
    let mut name = String::new();
    let line = string_reader.line();
    let pos = string_reader.pos();

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

    let token = match name.as_str() {
        "struct" => Token::Struct,
        "enum" => Token::Enum,
        "fn" => Token::Fn,
        "signal" => Token::Signal,
        "async" => Token::Async,
        "const" => Token::Const,
        "true" => Token::Literal(Literal::BoolLiteral(true)),
        "false" => Token::Literal(Literal::BoolLiteral(false)),
        _ => Token::ID { name },
    };

    TokenWithLineAndPos { line, pos, token }
}

fn lex_string(string_reader: &mut StringReader) -> TokenWithLineAndPos {
    let mut value = String::new();
    let line = string_reader.line();
    let pos = string_reader.pos();

    if string_reader.next().is_none() {
        let line = string_reader.line();
        let pos = string_reader.pos();

        panic!("{line}:{pos}: Expect string, but got EOF")
    }

    loop {
        let current = string_reader.current().unwrap();

        if current == '"' {
            string_reader.next();
            break;
        }

        value += &String::from(current);
        let next = string_reader.next();

        if next.is_some() {
            continue;
        } else {
            let line = string_reader.line();
            let pos = string_reader.pos();

            panic!("{line}:{pos}: Expect '\"', but got EOF");
        }
    }

    let token = Token::Literal(Literal::StringLiteral(value));

    TokenWithLineAndPos { line, pos, token }
}

fn lex_number(string_reader: &mut StringReader) -> TokenWithLineAndPos {
    let mut value = String::new();
    let mut is_hex = false;
    let mut is_float = false;
    let line = string_reader.line();
    let pos = string_reader.pos();

    loop {
        value += &String::from(string_reader.current().unwrap());
        let next = string_reader.next();

        match next {
            Some(ch) => {
                if ch == '_' {
                    string_reader.next();
                    continue;
                }

                if ch == '.' && !is_hex && !is_float {
                    is_float = true;
                    continue;
                }

                if ch == 'x' && !is_hex && !is_float {
                    is_hex = true;
                    continue;
                }

                if (!is_hex && !is_char_number(ch)) || (is_hex && !is_char_hex_number(ch)) {
                    break;
                }
            }
            None => break,
        }
    }

    let token = if is_hex {
        Token::Literal(Literal::IntLiteral(
            i64::from_str_radix(value.trim_start_matches("0x"), 16).unwrap(),
        ))
    } else if is_float {
        Token::Literal(Literal::NumberLiteral(value.parse::<f64>().unwrap()))
    } else {
        Token::Literal(Literal::IntLiteral(value.parse::<i64>().unwrap()))
    };

    TokenWithLineAndPos { line, pos, token }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_reader_simple() {
        let mut string_reader = StringReader::new("a");
        assert_eq!(string_reader.current(), Some('a'));
        assert_eq!(string_reader.line(), 1);
        assert_eq!(string_reader.pos(), 0);

        assert_eq!(string_reader.next(), None);
        assert_eq!(string_reader.line(), 1);
        assert_eq!(string_reader.pos(), 1);
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

        assert_eq!(string_reader.line(), 1);
        assert_eq!(string_reader.pos(), 6);
    }

    #[test]
    fn string_reader_line_break() {
        let mut string_reader = StringReader::new("abc123\r\ntest\ntest");
        assert_eq!(string_reader.current(), Some('a'));

        assert_eq!(string_reader.next(), Some('b'));
        assert_eq!(string_reader.line(), 1);
        assert_eq!(string_reader.pos(), 1);

        assert_eq!(string_reader.next(), Some('c'));
        assert_eq!(string_reader.next(), Some('1'));
        assert_eq!(string_reader.next(), Some('2'));
        assert_eq!(string_reader.next(), Some('3'));

        assert_eq!(string_reader.next(), Some('\r'));
        assert_eq!(string_reader.next(), Some('\n'));

        assert_eq!(string_reader.next(), Some('t'));
        assert_eq!(string_reader.line(), 2);
        assert_eq!(string_reader.pos(), 0);

        assert_eq!(string_reader.next(), Some('e'));
        assert_eq!(string_reader.line(), 2);
        assert_eq!(string_reader.pos(), 1);

        string_reader.next();
        string_reader.next();

        assert_eq!(string_reader.next(), Some('\n'));
        assert_eq!(string_reader.next(), Some('t'));
        assert_eq!(string_reader.line(), 3);
        assert_eq!(string_reader.pos(), 0);
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
    fn lex_symbols() {
        let mut lexer = Lexer::tokenize("#[]");
        let token = lexer.current_token();
        assert_eq!(token.clone(), Token::Symbol('#'));
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Symbol('['));
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Symbol(']'));
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    fn lex_comb() {
        let mut lexer = Lexer::tokenize("#[123]");
        let token = lexer.current_token();
        assert_eq!(token.clone(), Token::Symbol('#'));
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Symbol('['));
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Literal(Literal::IntLiteral(123)));
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Symbol(']'));
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
            Token::Literal(Literal::StringLiteral(String::from("hello     world")),)
        );
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    fn lex_int_literal() {
        let mut lexer = Lexer::tokenize("123 1_000_000 0xFF 0x0002_0006");
        let token = lexer.current_token();
        assert_eq!(token.clone(), Token::Literal(Literal::IntLiteral(123)));
        let token = lexer.next_token();
        assert_eq!(
            token.clone(),
            Token::Literal(Literal::IntLiteral(1_000_000))
        );
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Literal(Literal::IntLiteral(0xFF)));
        let token = lexer.next_token();
        assert_eq!(
            token.clone(),
            Token::Literal(Literal::IntLiteral(0x0002_0006))
        );
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    fn lex_number_literal() {
        let mut lexer = Lexer::tokenize("123. 342.23 03.001");
        let token = lexer.current_token();
        assert_eq!(token.clone(), Token::Literal(Literal::NumberLiteral(123.)));
        let token = lexer.next_token();
        assert_eq!(
            token.clone(),
            Token::Literal(Literal::NumberLiteral(342.23))
        );
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Literal(Literal::NumberLiteral(3.001)));
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    fn lex_bool_literal() {
        let mut lexer = Lexer::tokenize("true false");
        let token = lexer.current_token();
        assert_eq!(token.clone(), Token::Literal(Literal::BoolLiteral(true)));
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Literal(Literal::BoolLiteral(false)));

        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    fn lex_keywords() {
        let mut lexer = Lexer::tokenize("struct enum fn async signal const");
        let token = lexer.current_token();
        assert_eq!(token.clone(), Token::Struct);
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Enum);
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Fn);
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Async);
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Signal);
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Const);
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    fn lex_comments() {
        let mut lexer = Lexer::tokenize("struct /*enum*/ fn\n // Hello world\n enum");
        let token = lexer.current_token();
        assert_eq!(token.clone(), Token::Struct);
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Fn);
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::Enum);
        let token = lexer.next_token();
        assert_eq!(token.clone(), Token::EOF);
    }

    #[test]
    fn lex_doc_comments() {
        let mut lexer = Lexer::tokenize("/// Some documentation here\n///   Some Tab");
        let token = lexer.current_token();
        assert_eq!(
            token.clone(),
            Token::DocComment {
                value: String::from(" Some documentation here"),
            }
        );
        let token = lexer.next_token();
        assert_eq!(
            token.clone(),
            Token::DocComment {
                value: String::from("   Some Tab"),
            }
        );
    }
}
