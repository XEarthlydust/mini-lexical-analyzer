use std::cell::Cell;

use super::{results::result_println, words::WordType};

pub fn multilines_comment_start(line: &str, in_multiline_comment: &Cell<bool>) -> bool {
    if let Some(start_index) = line.find("/*") {
        if let Some(end_index) = line.find("*/") {
            if end_index > start_index {
                result_println(&WordType::Comments, "Multi Start&End");
                return true;
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
        if let Some(end_index) = line_without_comment.get().find("*/") {
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

pub fn single_comment_check (line_without_comment: &Cell<&str>){
    line_without_comment.set(if let Some(index) = line_without_comment.get().find("//") {
        result_println(&WordType::Comments, "Single Line");
        &line_without_comment.get()[..index]
    } else {
        line_without_comment.get()
    });
}