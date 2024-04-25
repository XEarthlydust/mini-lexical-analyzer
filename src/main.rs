mod tools;

use std::cell::Cell;
use std::fs;
use clap::Parser;

use tools::results::result_println;
use tools::lines::{
    cut_lines, multilines_comment_checkend, multilines_comment_start, single_comment_check,
};
use tools::words::Regexs;

#[derive(Parser)]
struct Args {
    /// The file path of *.c source.
    file: String,
}

fn main() {
    let regexs: Regexs = Regexs::new();
    let args = Args::parse();
    let in_multiline_comment = Cell::new(false);

    // 读取源文件
    let source_code = match fs::read_to_string(args.file) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read the source file.");
            return;
        }
    };
    
    // 按行读取
    for (el, line) in source_code.lines().enumerate() {
        println!("{}", { el+1 });

        let line_without_comment = Cell::new(line);
        // 检查&跳过 多行注释
        if multilines_comment_checkend(&line_without_comment, &in_multiline_comment)
            || multilines_comment_start(line, &in_multiline_comment)
        {
            continue;
        };
        // 检查&跳过 单行注释
        single_comment_check(&line_without_comment);
        // 按 界符&操作符 划分tokens
        let tokens = cut_lines(&line_without_comment);
        // 正则&输出
        for token in tokens {
            let word_type = regexs.matching(&token);
            result_println(word_type, &token)
        }
    }
}
