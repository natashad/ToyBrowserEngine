use std::collections::HashMap;
use std::io::Write;

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
}

pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node
    {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn element(name: String, attrs: AttrMap, children: Vec<Node> ) -> Node {
    Node
    {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

pub fn pretty_print(root: &Node)
{
    _pretty_print(root, 0);
}

fn _pretty_print(root: &Node, indent: i32)
{
    _print_indent(indent);
    let root = root.clone();
    match &root.node_type {
        &NodeType::Text(ref text) => print!("{}\n", text),
        &NodeType::Element(ref elem_data) => {
            print!("<{}>\n", elem_data.tag_name);
            for child in root.children.iter() {
                _pretty_print(child, indent+1);
            }
            _print_indent(indent);
            print!("</{}>\n", elem_data.tag_name);
        }
    }
    ::std::io::stdout().flush().unwrap();

}

fn _print_indent(indent: i32) {
    for _ in 0..indent {
        print!("    ");
        ::std::io::stdout().flush().unwrap();
    }
}