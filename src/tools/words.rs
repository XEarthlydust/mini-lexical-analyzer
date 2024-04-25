pub enum WordType {
    Comments,
    Keyword,
    Identifier,
    Delimiter,
    Operator,
    Literal(LiteralType),
}

pub enum LiteralType {
    String,
    Char,
    Integer,
    Float,
}
