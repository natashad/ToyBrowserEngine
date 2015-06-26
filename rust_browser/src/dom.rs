use std::collections::HashMap;

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
    for _ in 0..indent {
        println!("    ");
    }

    let root = root.clone();
    match &root.node_type {
        &NodeType::Text(ref text) => println!("{}", text),
        &NodeType::Element(ref elem_data) => {
            println!("<{}>", elem_data.tag_name);
            for child in root.children.iter() {
                _pretty_print(child, indent-1);
            }
            println!("</{}>", elem_data.tag_name);
        }
    }
    println!("");
}