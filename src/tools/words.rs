use std::backtrace;

use regex::Regex;

pub enum WordType {
    Comments,
    Keyword,
    Identifier,
    Delimiter,
    Operator,
    Literal(LiteralType),
    Notfound,
}

pub enum LiteralType {
    StringRaw,
    CharRaw,
    String,
    Char,
    Integer,
    Float,
}

pub fn is_delimiter(c: char) -> bool {
    "()[]{};,.".contains(c)
}

pub fn is_operator(c: char) -> bool {
    "+-*/%=><!&|^".contains(c)
}

pub struct Regexs<'a> {
    // keyword: Regex,
    // identifier: Regex,
    // string_literal: Regex,
    // char_literal: Regex,
    // integer_literal: Regex,
    // float_literal: Regex,
    // delimiter: Regex,
    // operator: Regex,
    regex_vec: Vec<(Regex, &'a WordType)>,
}

impl<'a> Regexs<'a> {
    pub fn init() -> Regexs<'a> {
        let keyword_pattern = r"\b(auto|break|case|char|const|continue|default|do|double|else|enum|extern|float|for|goto|if|int|long|register|return|short|signed|sizeof|static|struct|switch|typedef|union|unsigned|void|volatile|while)\b";
        let identifier_pattern = r"[a-zA-Z_]\w*";
        let string_literal_pattern = r#""(?:\\.|[^\\"])*""#;
        let char_literal_pattern = r"'(?:\\.|[^\\'])'";
        let integer_literal_pattern = r"\b\d+\b";
        let float_literal_pattern = r"\b\d+\.\d+\b";
        let delimiter_pattern = r"[()\[\]{};,]";
        let operator_pattern = r"[\+\-\*/%=><!&\|\^]";

        // 生成正则表达式
        let keyword = Regex::new(keyword_pattern).unwrap();
        let identifier = Regex::new(identifier_pattern).unwrap();
        let string_literal = Regex::new(string_literal_pattern).unwrap();
        let char_literal = Regex::new(char_literal_pattern).unwrap();
        let integer_literal = Regex::new(integer_literal_pattern).unwrap();
        let float_literal = Regex::new(float_literal_pattern).unwrap();
        let delimiter = Regex::new(delimiter_pattern).unwrap();
        let operator = Regex::new(operator_pattern).unwrap();

        Regexs {
            regex_vec: vec![
                (keyword, &WordType::Keyword),
                (operator, &WordType::Operator),
                (string_literal, &WordType::Literal(LiteralType::StringRaw)),
                (char_literal, &WordType::Literal(LiteralType::CharRaw)),
                (integer_literal, &WordType::Literal(LiteralType::Integer)),
                (float_literal, &WordType::Literal(LiteralType::Float)),
                (delimiter, &WordType::Delimiter),
                (identifier, &WordType::Identifier),
            ],
        }
    }

    pub fn matching(&self, token: &str) -> &WordType {
        for (c, d) in &self.regex_vec {
            if c.is_match(&token) {
                return d;
            }
        }
        &WordType::Notfound
    }
}
