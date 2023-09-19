use scupt_util::res::Res;
use tree_sitter::{Parser, Tree};

pub struct TLAVarsParser {
    parser: Parser,
}

impl TLAVarsParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_label_tla::language()).unwrap();
        Self {
            parser
        }
    }

    pub fn parse(&mut self, text: &String) -> Res<Tree> {
        let tree = self.parser.parse(text, None).expect("parse");
        Ok(tree)
    }
}

