//! HTML parser
//!
//! TODO:
//! * Comments
//! * Doctype declarations
//! * Self-closing tags
//! * Error handling (e.g. unbalanced tags)
//! * Namespaces and other XHTML syntax
//! * Character encoding detection

use dom;
use std::collections::HashMap;


struct Parser {
    pos: usize,
    input: String
}

impl Parser {
    // Parser methods for peeking at next chars in input
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Check if next characters start with given string
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // Check if at end of file
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // Return current char, then advance self.pos to the next char
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (, cur_char) = iter.next().unwrap();
        let (next_pos, ) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    // Consume until test function returns false on char
    fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    // Consume zero or more whitespace chars
    fn consume_whitespace(&mut self) {
        self.consume_while(CharExt::is_whitespace);
    }

    // Pare tag or attribute name of alphanumeric chars
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false
        })
    }

    /* Node parsing methods */

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text()
        }
    }

    // Parse text node until opening tag
    fn parse_text(&mut self) -> dom::Node {
        dom::text_node(self.consume_while(|c| c != '<'))
    }

    // Parse element node including open tag, contents and closing tag
    fn parse_element(&mut self) -> dom::Node {
        // Open tag
        assert!(self.consume_char() == '<'); // TODO: Some of these asserts should be more forgiving
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Contents
        let children = self.parse_nodes();

        // Closing tag
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::element_node(tag_name, attrs, children);
    }

    // name="value" (name, value) pair
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    // Parse quoted value
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == "&#39");
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    // Parse list of name="value"s separated by whitespace
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    // Loop and accumulate child nodes until closing tag
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }

    // Parse HTML document starting with root node
    // Will create root node if one is not found
    pub fn parse(source: String) -> dom::Node {
        let mut nodes = Parser {
            pos: 0,
            input: source
        }.parse_nodes();

        if nodes.len() == 1 {
            nodes.swap_remove(0);
        } else {
            dom::element_node("html".to_string(), HashMap::new(), nodes)
        }
    }
}
