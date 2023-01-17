use std::io;
use std::collections::HashMap;
use crate::types::*;

pub struct Lexer {
    line: u32,
    peek: [u8;1],
    words: HashMap<String, Token>,
}

impl Lexer {
    fn reserve(&mut self, word: Token, idx: String) {
        self.words.insert(idx, word);
    }

    pub fn new() -> Self {
        let mut lexer = Lexer {
            line: 1,
            peek: [b' '],
            words: HashMap::new()
        };

        lexer.reserve(Token::TRUE, "true".to_string());
        lexer.reserve(Token::FALSE, "false".to_string());
        lexer
    }

    fn scan<T: io::Read>(&mut self, mut buf: T) -> io::Result<Token> {
        while WHITE_SPACES.contains(self.peek[0] as char) {
            if self.peek[0] as char == '\n' { self.line += 1; }
            buf.read(&mut self.peek)?;
        }

        let chr = self.peek[0] as char;
        match chr {
            // possible a comment
            _ if SYMBOL_CHARS.contains(chr) => {
                let mut sym = String::new();
                let mut ch1 = chr;
                buf.read(&mut self.peek)?;
                let mut ch2 = self.peek[0] as char;
                // single line comment
                if ch1 == '/' && ch2 == '/' {
                    while ch1 != '\n' {
                        buf.read(&mut self.peek)?;
                        ch1 = self.peek[0] as char;
                    }
                    return Ok(Token::COMMENT);
                }
                // multi line comment
                if ch1 == '/' && ch2 == '*' {
                    while !(ch1 == '*' && ch2 == '/') {
                        ch1 = ch2;
                        buf.read(&mut self.peek)?;
                        ch2 = self.peek[0] as char;
                    }
                    return Ok(Token::COMMENT);
                }
                // symbol
                while SYMBOL_CHARS.contains(ch2) {
                    if ch1 == '\\' && ch2 == '\\' {
                        return Ok(Token::SYMBOL(sym));
                    }
                    sym.push(ch1);
                    ch1 = ch2;
                    buf.read(&mut self.peek)?;
                    ch2 = self.peek[0] as char;
                }
                sym.push(ch1);
                Ok(Token::SYMBOL(sym))
            }
            // num literals
            _ if NUM_START.contains(chr) => {
                let mut num = 0;
                let mut pk = chr;
                while NUM_CHARS.contains(pk) {
                    if let Some(x) = pk.to_digit(10) {
                        num = num * 10 + x as i32;
                    }
                    buf.read(&mut self.peek)?;
                    pk = self.peek[0] as char;
                }
                if pk == '.' {
                    buf.read(&mut self.peek)?;
                    pk = self.peek[0] as char;
                    let mut pow = -1;
                    let mut num = num as f32;
                    while NUM_CHARS.contains(pk) {
                        if let Some(x) = pk.to_digit(10) {
                            num += (x as f32) * f32::powi(10.0, pow);
                        }
                        pow -= 1;
                        buf.read(&mut self.peek)?;
                        pk = self.peek[0] as char;
                    }
                    Ok(Token::FLOAT(num))
                } else {
                    Ok(Token::INT(num))
                }
            },
            // identifiers
            _ if VAR_START.contains(chr) => {
                let mut var = String::new();
                let mut pk = chr;
                while VAR_CHARS.contains(pk) {
                    var.push(pk);
                    buf.read(&mut self.peek)?;
                    pk = self.peek[0] as char;
                }
                if let Some(tkn) = self.words.get(&var).cloned() {
                    Ok(tkn)
                } else {
                    Ok(Token::VAR(var))
                }
            },
            _ => {
                self.peek[0] = b' ';
                Ok(Token::UNID(chr))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::types::*;
    use std::fs::File;
    use std::io;

    #[test]
    fn comment() -> io::Result<()>{
        let f = File::open("tests/comment")?;
        let mut lexer = Lexer::new();
        let token = lexer.scan(f)?;
        match token {
            Token::COMMENT => assert_eq!(2, 2),
            _ => panic!("{token:?}"),
        }
        Ok(())
    }

    #[test]
    fn integer() -> io::Result<()>{
        let f = File::open("tests/int")?;
        let mut lexer = Lexer::new();
        let token = lexer.scan(f)?;
        match token {
            Token::INT(x) => assert_eq!(x, 23456),
            _ => panic!("Did not parse the integer correctly"),
        }
        assert_eq!(2, 2);
        Ok(())
    }

    #[test]
    fn float() -> io::Result<()>{
        let f = File::open("tests/float")?;
        let mut lexer = Lexer::new();
        let token = lexer.scan(f)?;
        match token {
            Token::FLOAT(x) => assert_eq!(x.to_string(), "3389.34".to_string()),
            _ => panic!("Did not parse the float correclty"),
        }
        assert_eq!(2, 2);
        Ok(())
    }

    #[test]
    fn var() -> io::Result<()>{
        let f = File::open("tests/var")?;
        let mut lexer = Lexer::new();
        let token = lexer.scan(f)?;
        match token {
            Token::VAR(x) => assert_eq!(x, "fgujnmk".to_string()),
            _ => panic!("Did not parse the word correclty"),
        }
        assert_eq!(2, 2);
        Ok(())
    }
    
    #[test]
    fn symbol() -> io::Result<()> {
        let f = File::open("tests/symbol")?;
        let mut lexer = Lexer::new();
        let token = lexer.scan(f)?;
        match token {
            Token::SYMBOL(x) => assert_eq!(x, ">>=".to_string()),
            _ => panic!("{token:?}"),
        }
        assert_eq!(2, 2);
        Ok(())
    }

    #[test]
    fn true_test() -> io::Result<()> {
        let f = File::open("tests/true")?;
        let mut lexer = Lexer::new();
        let token = lexer.scan(f)?;
        match token {
            Token::TRUE => assert_eq!(2, 2),
            _ => panic!("{token:?}")
        }
        Ok(())
    }
}
