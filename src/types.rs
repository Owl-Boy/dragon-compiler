/* #[derive(Debug, Clone, Copy)]
pub enum Tag {
    NUM,
    VAR,
    RES,
    TRUE,
    FALSE,
}

pub trait Token {
    fn get_tag(&self) -> Tag;
}

pub struct Int {
    value: i32,
}

impl Token for Int {
    fn get_tag(&self) -> Tag {
        Tag::NUM
    }
}

pub struct Word {
    pub lexeme: String,
    pub tag: Tag,
}

impl Word {
    pub fn new(lexeme: String, tag: Tag) -> Self {
        Word { lexeme, tag }
    }
}

impl Token for Word {
    fn get_tag(&self) -> Tag {
        self.tag
    }
} */
pub const NUM_CHARS: &str = "1234567890_";
pub const NUM_START: &str = "1234567890";
pub const VAR_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890_";
pub const VAR_START: &str =  "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_";
pub const WHITE_SPACES: &str = " \n\r";
pub const SYMBOL_CHARS: &str = "\\/!@#$%^&*?~<>=";

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    INT(i32),
    FLOAT(f32),
    VAR(String),
    STRING(String),
    SYMBOL(String),
    TRUE, FALSE,
    UNID(char),
    COMMENT
}
