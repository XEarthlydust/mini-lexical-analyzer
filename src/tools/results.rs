use super::words::{WordType, LiteralType};

// 辅助格式化结果输出
pub fn result_println(word_type: &WordType, token: &str) {
    match word_type {
        WordType::Comments => {
            println!("{:20}{}", "  CodeComments", token);
        }
        WordType::Keyword => {
            println!("{:20}{}", "  Keyword:", token);
        }
        WordType::Delimiter => {
            println!("{:20}{}", "  Delimiter:", token);
        }
        WordType::Operator => {
            println!("{:20}{}", "  Operator:", token);
        }
        WordType::Literal(LiteralType::Char) => {
            println!("{:20}{}", "  Char Literal:", token);
        }
        WordType::Literal(LiteralType::Float) => {
            println!("{:20}{}", "  Float Literal:", token);
        }
        WordType::Literal(LiteralType::Integer) => {
            println!("{:20}{}", "  Integer Literal:", token);
        }
        WordType::Literal(LiteralType::String) => {
            println!("{:20}{}", "  String Literal:", token);
        }
        WordType::Identifier => {
            println!("{:20}{}", "  Identifier:", token);
        }
        // _ => println!("{:20}{}", "  NotMatch:", token),
    }
}

//拆分字符串&单字符的界符
pub fn char_string_outdelimiter(word_type: &WordType, token: &str) {
    let token = String::from(token);
    result_println(&WordType::Keyword, &token[..0]);
    if token.len() > 2 {
        result_println(word_type, &token[1..token.len() - 1]);
    } else {
        result_println(word_type, &String::from("Null Character String!"));
    }
    result_println(&WordType::Keyword, &token[token.len()..]);
}