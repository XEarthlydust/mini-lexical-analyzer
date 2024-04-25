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

pub struct Regexs {
    keyword: Regex,
    identifier: Regex,
    string_literal: Regex,
    char_literal: Regex,
    integer_literal: Regex,
    float_literal: Regex,
    delimiter: Regex,
    operator: Regex,
}

impl Regexs {
    pub fn init() -> Regexs {
        let keyword_pattern = r"\b(auto|break|case|char|const|continue|default|do|double|else|enum|extern|float|for|goto|if|int|long|register|return|short|signed|sizeof|static|struct|switch|typedef|union|unsigned|void|volatile|while)\b";
        let identifier_pattern = r"[a-zA-Z_]\w*";
        let string_literal_pattern = r#""(?:\\.|[^\\"])*""#;
        let char_literal_pattern = r"'(?:\\.|[^\\'])'";
        let integer_literal_pattern = r"\b\d+\b";
        let float_literal_pattern = r"\b\d+\.\d+\b";
        let delimiter_pattern = r"[()\[\]{};,]";
        let operator_pattern = r"[\+\-\*/%=><!&\|\^]";

        Regexs {
            keyword: Regex::new(keyword_pattern).unwrap(),
            identifier: Regex::new(identifier_pattern).unwrap(),
            string_literal: Regex::new(string_literal_pattern).unwrap(),
            char_literal: Regex::new(char_literal_pattern).unwrap(),
            integer_literal: Regex::new(integer_literal_pattern).unwrap(),
            float_literal: Regex::new(float_literal_pattern).unwrap(),
            delimiter: Regex::new(delimiter_pattern).unwrap(),
            operator: Regex::new(operator_pattern).unwrap(),
        }
        // 生成正则表达式

    }

    pub fn matching(&self, token: &str) -> WordType {
        if self.keyword.is_match(&token) {
            WordType::Keyword
        } else if self.operator.is_match(&token) {
            WordType::Operator
        } else if self.string_literal.is_match(&token) {
            WordType::Literal(LiteralType::StringRaw)
        } else if self.char_literal.is_match(&token) {
            WordType::Literal(LiteralType::CharRaw)
        } else if self.integer_literal.is_match(&token) {
            WordType::Literal(LiteralType::Integer)
        } else if self.float_literal.is_match(&token) {
            WordType::Literal(LiteralType::Float)
        } else if self.delimiter.is_match(&token) {
            WordType::Delimiter
        } else if self.identifier.is_match(&token) {
            WordType::Identifier
        } else {
            WordType::Notfound
        }
    }
}
