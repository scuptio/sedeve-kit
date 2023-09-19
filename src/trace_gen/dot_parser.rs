use scupt_util::res::Res;
use tree_sitter::{Parser, Tree};

pub struct DotParser {
    parser: Parser,
}

impl DotParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_label_tla::language()).unwrap();
        parser.set_language(tree_sitter_dot::language()).unwrap();
        Self {
            parser
        }
    }

    pub fn parse(&mut self, text: &String) -> Res<Tree> {
        let tree = self.parser.parse(text, None).expect("parse");
        Ok(tree)
    }
}

