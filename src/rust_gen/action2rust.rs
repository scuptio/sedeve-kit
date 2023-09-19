extern crate core;

use std::fmt::Write;
use std::fs;
use std::fs::read_to_string;
use std::path::Path;

use scupt_util::error_type::ET;
use scupt_util::res::Res;
use scupt_util::res_of::res_io;
use tracing::trace;

use crate::rust_gen::action_visitor::ActionVisitor;
use crate::rust_gen::ast2rust::{A2RContext, AST2Rust};

const BEGIN_ACTION: &str = "(*@begin@";
const END_ACTION: &str = "@end@*)";


fn find_between_markers(input: &str) -> Vec<String> {
    let begin_marker = BEGIN_ACTION;
    let end_marker = END_ACTION;
    let mut result = Vec::new();
    let mut start = input.find(begin_marker);

    while let Some(begin) = start {
        let begin = begin + begin_marker.len();
        if let Some(end) = input[begin..].find(end_marker) {
            let end = begin + end;
            result.push(input[begin..end].to_string());
            start = input[end + end_marker.len()..].find(begin_marker).map(
                |i| i + end + end_marker.len());
        } else {
            break;
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_between_markers() {
        let input = format!("Some text {}first{} and {}second{} more text",
                            BEGIN_ACTION,
                            END_ACTION,
                            BEGIN_ACTION,
                            END_ACTION
        );
        let expected = vec!["first".to_string(), "second".to_string()];
        assert_eq!(find_between_markers(input.as_str()), expected);

        let input = "No markers here".to_string();
        let expected: Vec<String> = Vec::new();
        assert_eq!(find_between_markers(input.as_str()), expected);

        let input = format!("{}only begin", BEGIN_ACTION);
        let expected: Vec<String> = Vec::new();
        assert_eq!(find_between_markers(input.as_str()), expected);

        let input = format!("{}only end", END_ACTION);
        let expected: Vec<String> = Vec::new();
        assert_eq!(find_between_markers(input.as_str()), expected);

        let input = format!("{}{}empty{}{}",
                            BEGIN_ACTION, END_ACTION,
                            BEGIN_ACTION, END_ACTION);
        let expected = vec!["".to_string(), "".to_string()];
        assert_eq!(find_between_markers(input.as_str()), expected);
    }
}


fn file_to_action_content(path: String, is_tla_file: bool) -> Res<String> {
    let r_file_contents = read_to_string(path);
    let mut contents = res_io(r_file_contents)?;
    if is_tla_file {
        let vec = find_between_markers(contents.as_str());
        let mut ss = String::new();
        for s in vec {
            write!(&mut ss, "{}\n", s).unwrap();
        }
        contents = ss;
    }

    Ok(contents)
}


pub fn action_to_rust(path: String, output: String) -> Res<()> {
    let p = Path::new(path.as_str());

    return if let Some(ext) = p.extension() {
        let s = ext.to_str().unwrap();
        let content = if s == "tla" {
            file_to_action_content(path, true)?
        } else if s == "action" {
            file_to_action_content(path, true)?
        } else {
            return Err(ET::IOError("unknown extension".to_string()))
        };
        trace!("{}", content);
        let mut visitor = ActionVisitor::new(content);
        let def = visitor.parse()?;
        let mut ctx = A2RContext::new();
        let source = def.to_rust_code(&mut ctx)?;
        let fmt_source = ActionVisitor::format_code(&source).unwrap();
        let r_write = fs::write(output, fmt_source);
        res_io(r_write)?;
        Ok(())
    } else {
        Err(ET::IOError("cannot open file".to_string()))
    }
}

