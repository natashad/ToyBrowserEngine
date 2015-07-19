use std::collections::HashMap;
use dom;
use css_parser;

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a dom::Node,
    pub css_values: CSSProperties,
    pub children: Vec<StyledNode<'a>>,
}

pub type CSSProperties = HashMap<String, css_parser::CSSValue>;

pub fn build_style_tree<'a>(root: &'a dom::Node, stylesheet: &'a css_parser::Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        css_values: match root.node_type {
            dom::NodeType::Element(ref elem_data) => get_properties(elem_data, stylesheet),
            dom::NodeType::Text(_) => HashMap::new()
        },
        children: root.children.iter().map(|child| build_style_tree(child, stylesheet)).collect(),
    }
}

type MatchingRule<'a> = (css_parser::Specificity, &'a css_parser::Rule);

fn get_properties(elem: &dom::ElementData, stylesheet: &css_parser::Stylesheet) -> CSSProperties {

    let mut values = HashMap::new();
    let mut rules = get_matching_rules(elem, stylesheet);

    rules.sort_by(|&(a,_), &(b, _)| a.cmp(&b));

    for (_, rule) in rules {
        for decl in &rule.declarations {
            values.insert(decl.name.clone(), decl.value.clone());
        }
    }
    return values;
}

fn get_matching_rules<'a>(elem: &dom::ElementData, stylesheet : &'a css_parser::Stylesheet) -> Vec<MatchingRule<'a>> {
    stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}

fn match_rule<'a>(elem: &dom::ElementData, rule: &'a css_parser::Rule) -> Option<MatchingRule<'a>> {
    rule.selectors.iter().find(|selector| match_selectors(elem, *selector)).map(|selector|(selector.specificity(), rule))
}

fn match_selectors(elem: &dom::ElementData, selector: &css_parser::Selector) -> bool {
    match *selector {
        css_parser::Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector)
    }
}

fn matches_simple_selector(elem: &dom::ElementData, selector: &css_parser::SimpleSelector) -> bool{

    if selector.tag.iter().any(|tag| elem.tag_name != *tag) {
        return false;
    }

    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    if selector.class.iter().any(|class| !elem.classes().contains(&**class)) {
        return false;
    }
    return true;
}