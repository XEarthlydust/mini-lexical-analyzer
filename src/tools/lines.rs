use std::cell::Cell;

use super::words::{is_delimiter, is_operator};
use super::{results::result_println, words::WordType};

pub fn multilines_comment_start(
    line_without_comment: &Cell<&str>,
    in_multiline_comment: &Cell<bool>,
) -> bool {
    if let Some(start_index) = line_without_comment.get().find("/*") {
        if let Some(end_index) = line_without_comment.get().rfind("*/") {
            if end_index > start_index {
                result_println(&WordType::Comments, "Multi Start&End");
                line_without_comment.set(&line_without_comment.get()[end_index + 2..]);
                return false;
            }
        }
        result_println(&WordType::Comments, "Multi Start");
        in_multiline_comment.set(true);
        return true;
    } else {
        return false;
    }
}

pub fn multilines_comment_checkend(
    line_without_comment: &Cell<&str>,
    in_multiline_comment: &Cell<bool>,
) -> bool {
    if in_multiline_comment.get() {
        if let Some(end_index) = line_without_comment.get().rfind("*/") {
            result_println(&WordType::Comments, "Multi End");
            in_multiline_comment.set(false);
            line_without_comment.set(&line_without_comment.get()[end_index + 2..]);
        } else {
            result_println(&WordType::Comments, "Multi Line");
            return true;
        }
    }
    return false;
}

pub fn single_comment_check(line_without_comment: &Cell<&str>) {
    line_without_comment.set(if let Some(index) = line_without_comment.get().find("//") {
        result_println(&WordType::Comments, "Single Line");
        &line_without_comment.get()[..index]
    } else {
        line_without_comment.get()
    });
}

// 拆分代码行成单词和界符
pub fn cut_lines(line_without_comment: &Cell<&str>) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    for c in line_without_comment.get().chars() {
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
    return tokens;
}
