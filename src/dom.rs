/* DOM node base structures */

use std::collections::HashMap;


pub type AttrMap = HashMap<String, String>;

pub struct Node {
    children: Vec<Node>,
    node_type: NodeType
}

pub enum NodeType {
    Text(String),
    Element(ElementData)
}

pub struct ElementData {
    tag_name: String,
    attributes: AttrMap
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

pub fn pretty_print_nodes() {
    println!("Nodes!");
}
