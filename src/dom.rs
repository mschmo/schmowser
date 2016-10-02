//! DOM node base structures

use std::collections::HashMap;


pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData)
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap
}


/* New Node Funcs */

pub fn text_node(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data)
    }
}

pub fn element_node(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs
        })
    }
}

/* Misc. Helper Funcs */

pub fn pretty_print_node(root_node: Node) {
    println!("{:?}", root_node);
}
