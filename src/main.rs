mod dom;


fn main() {
    let node = dom::Node{
        children: vec![],
        node_type: dom::NodeType::Text("I am a text node".to_string())
    };
    dom::pretty_print_node(node);
}
