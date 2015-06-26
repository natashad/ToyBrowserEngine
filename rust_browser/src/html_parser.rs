use dom;

pub fn parse(input : String) -> dom::Node {
    let mut nodes = Parser {pos : 0, input: input}.parse_node_sequence();
    if nodes.len() == 1 {
        nodes.remove(0)
    } else {
        dom::element("html".to_string(), dom::AttrMap::new(), nodes)
    }
}

struct Parser {
    input: String,
    pos: usize
}

impl Parser {

    fn parse_element(&mut self) -> dom::Node {
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_or_attr_name();
        let attrs = self.parse_attrs();
        assert!(self.consume_char() == '>');

        let children = self.parse_node_sequence();

        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_or_attr_name() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::element(tag_name, attrs, children);
    }

    fn parse_node_sequence(&mut self) -> Vec<dom::Node> {
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

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _   => self.parse_text()
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_tag_or_attr_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false
        })
    }

    fn parse_attrs(&mut self) -> dom::AttrMap {
        let mut attrs = dom::AttrMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let name = self.parse_tag_or_attr_name();
            assert!(self.consume_char() == '=');
            let quote = self.consume_char();
            assert!(quote == '"' || quote == '\'');
            let val = self.consume_while(|c| c!=quote);
            assert!(self.consume_char() == quote);
            attrs.insert(name, val);
        }
        return attrs;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn consume_while<Func>(&mut self, f: Func) -> String
        where Func: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && f(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn consume_char(&mut self) -> char {
        let mut it = self.input[self.pos..].char_indices();
        let (_, ret) = it.next().unwrap();
        let (next_pos, _) = it.next().unwrap();
        self.pos += next_pos;
        return ret;
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}