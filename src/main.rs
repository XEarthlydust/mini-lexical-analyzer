use regex::Regex;
use std::fs;

fn main() {
    // 读取源文件
    let source_code = match fs::read_to_string("main.c") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read the source file.");
            return;
        }
    };

    // 定义正则表达式模式
    let keyword_pattern = r"\b(auto|break|case|char|const|continue|default|do|double|else|enum|extern|float|for|goto|if|int|long|register|return|short|signed|sizeof|static|struct|switch|typedef|union|unsigned|void|volatile|while)\b";
    let identifier_pattern = r"[a-zA-Z_]\w*";
    let string_literal_pattern = r#""(?:\\.|[^\\"])*""#;
    let char_literal_pattern = r"'(?:\\.|[^\\'])'";
    let integer_literal_pattern = r"\b\d+\b";
    let float_literal_pattern = r"\b\d+\.\d+\b";
    let delimiter_pattern = r"[()\[\]{};,]";
    let operator_pattern = r"[\+\-\*/%=><!&\|\^]";

    // 编译正则表达式
    let keyword_regex = Regex::new(keyword_pattern).unwrap();
    let identifier_regex = Regex::new(identifier_pattern).unwrap();
    let string_literal_regex = Regex::new(string_literal_pattern).unwrap();
    let char_literal_regex = Regex::new(char_literal_pattern).unwrap();
    let integer_literal_regex = Regex::new(integer_literal_pattern).unwrap();
    let float_literal_regex = Regex::new(float_literal_pattern).unwrap();
    let delimiter_regex = Regex::new(delimiter_pattern).unwrap();
    let operator_regex = Regex::new(operator_pattern).unwrap();

    // 逐行分析源代码
    let mut in_multiline_comment = false;
    for line in source_code.lines() {
        let mut line_without_comment = line;
        if in_multiline_comment {
            if let Some(end_index) = line.find("*/") {
                in_multiline_comment = false;
                line_without_comment = &line[end_index + 2..];
            } else {
                continue;
            }
        }

        // 移除单行注释
        let line_without_comment = if let Some(index) = line_without_comment.find("//") {
            &line_without_comment[..index]
        } else {
            line_without_comment
        };

        // 拆分代码行成单词和界符
        let mut tokens = Vec::new();
        let mut token = String::new();
        for c in line_without_comment.chars() {
            if c.is_whitespace() || is_delimiter(c) || is_operator(c) {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                if is_delimiter(c) || is_operator(c) {
                    tokens.push(c.to_string());
                }
            } else {
                token.push(c);
            }
        }
        if !token.is_empty() {
            tokens.push(token);
        }

        // 多行注释检测
        if let Some(start_index) = line.find("/*") {
            if let Some(end_index) = line.find("*/") {
                if end_index > start_index {
                    continue;
                }
            }
            in_multiline_comment = true;
        }

        // 逐个匹配单词、界符和操作符
        for token in tokens {
            if keyword_regex.is_match(&token) {
                println!("Keyword: {}", token);
            } else if operator_regex.is_match(&token) {
                println!("Operator: {}", token);
            } else if string_literal_regex.is_match(&token) {
                println!("String Literal: {}", token);
            } else if char_literal_regex.is_match(&token) {
                println!("Char Literal: {}", token);
            } else if integer_literal_regex.is_match(&token) {
                println!("Integer Literal: {}", token);
            } else if float_literal_regex.is_match(&token) {
                println!("Float Literal: {}", token);
            } else if delimiter_regex.is_match(&token) {
                println!("Delimiter: {}", token);
            } else if identifier_regex.is_match(&token) {
                println!("Identifier: {}", token);
            }
        }
    }
}

fn is_delimiter(c: char) -> bool {
    "()[]{};,.".contains(c)
}

fn is_operator(c: char) -> bool {
    "+-*/%=><!&|^".contains(c)
}
