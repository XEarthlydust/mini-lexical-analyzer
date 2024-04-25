mod tools;

use std::cell::Cell;
use std::fs;

use clap::Parser;
use tools::results::result_println;
use tools::words::{is_delimiter, is_operator, WordType};

use crate::tools::lines::{multilines_comment_checkend, multilines_comment_start};
use crate::tools::words::Regexs;

#[derive(Parser)]
struct Args {
    /// The file path of *.c source.
    file: String,
}

fn main() {
    let args = Args::parse();

    // 读取源文件
    let source_code = match fs::read_to_string(args.file) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read the source file.");
            return;
        }
    };

    // 逐行分析源代码
    let in_multiline_comment = Cell::new(false);

    for (el, line) in source_code.lines().enumerate() {
        println!("{}", { el });
        let line_without_comment = Cell::new(line);

        if multilines_comment_checkend(&line_without_comment, &in_multiline_comment)
            || multilines_comment_start(line, &in_multiline_comment)
        {
            continue;
        };

        // 过滤单行注释
        let line_without_comment = if let Some(index) = line_without_comment.get().find("//") {
            result_println(&WordType::Comments, "Single Line");
            &line_without_comment.get()[..index]
        } else {
            line_without_comment.get()
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

        // 逐个匹配单词、界符和操作符
        let regexs = Regexs::init();
        for token in tokens {
            let word_type = regexs.matching(&token);
            result_println(word_type, &token)
        }
    }
}
