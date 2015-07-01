#[derive(Debug)]
pub struct Stylesheet {
    pub rules : Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub selectors : Vec<Selector>,
    pub declarations : Vec<Declaration>,
}

#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
    pub tag : Option<String>,
    pub id : Option<String>,
    pub class : Vec<String>,
}

#[derive(Debug)]
pub struct Declaration {
    pub name : String,
    pub value : CSSValue,
}

#[derive(Debug)]
pub enum CSSValue {
    Length(f32, Unit),
    ColorVal(Color),
    Keyword(String),
}

#[derive(Debug)]
pub enum Unit {
    Px,
}

#[derive(Debug)]
pub struct Color {
    pub r : u8,
    pub g : u8,
    pub b : u8,
    pub a : u8,
}

pub type Specificity = (usize, usize, usize);

impl Stylesheet {
    pub fn pretty_print(&self)  {
        for rule in self.rules.iter() {
            println!("{:?}", rule);
        }
    }
}

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref selector) = *self;
        let tcount = selector.tag.iter().count();
        let icount = selector.id.iter().count();;
        let ccount = selector.class.len();
        // Order of specificity: id, class, tag.
        (icount, ccount, tcount)
    }
}

impl CSSValue {
    pub fn px_value(&self) -> f32 {
        match *self {
            CSSValue::Length(length, Unit::Px) => length,
            _ => 0.0
        }
    }
}

// Parsing.

struct Parser {
    pos : usize,
    input : String,
}


pub fn parse(input: String) -> Stylesheet {
    let mut parser = Parser { input: input, pos: 0 };
    Stylesheet { rules : parser.parse_rules() }
}

impl Parser {


    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() { break; }
            rules.push(self.parse_rule());
        }
        return rules;
    }

    fn parse_rule(&mut self) -> Rule {
        Rule { selectors : self.parse_selectors(), declarations: self.parse_declarations() }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' =>  {
                            self.consume_char();
                            self.consume_whitespace();
                        }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c)
            }
        }
        selectors.sort_by(|a,b| b.specificity().cmp(&a.specificity()));
        return selectors;
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector { tag : None, id : None, class : Vec::new() };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag = Some(self.parse_identifier());
                }
                _ => break
            }
        }
        return selector;
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert!(self.consume_char() == '{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_single_declaration());
        }
        return declarations;
    }

    fn parse_single_declaration(&mut self) -> Declaration {

        let property = self.parse_identifier();
        self.consume_whitespace();
        assert!(self.consume_char() == ':');
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        assert!(self.consume_char() == ';');
        Declaration {
            name: property,
            value: value,
        }
    }

    fn parse_value(&mut self) -> CSSValue {
        match self.next_char() {
            '0'...'9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => CSSValue::Keyword(self.parse_identifier())
        }
    }

    fn parse_length(&mut self) -> CSSValue {
        let float = self.consume_while(|c| match c { '0'...'9' | '.' => true, _ => false});
        assert!(self.consume_char() == 'p');
        assert!(self.consume_char() == 'x');
        CSSValue::Length (float.parse().unwrap(), Unit::Px)
    }

    fn parse_color(&mut self) -> CSSValue {

        assert!(self.consume_char() == '#');
        CSSValue::ColorVal(Color{
            r : self.parse_hex_pair(),
            g : self.parse_hex_pair(),
            b : self.parse_hex_pair(),
            a : 255})
    }

    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.input[self.pos .. self.pos+2];
        self.pos = self.pos+2;
        u8::from_str_radix(s, 16).unwrap()
    }


    ///////////////////////
    // General Functions //
    ///////////////////////

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn consume_char(&mut self) -> char {
        let mut indices = self.input[self.pos..].char_indices();
        let (_, next_char) = indices.next().unwrap();
        let (next_pos, _) = indices.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return next_char;
    }

    fn  consume_while<Func>(&mut self, f:Func) -> String
        where Func: Fn(char) -> bool {

        let mut result = String::new();

        while !self.eof() && f(self.next_char()) {
            result.push(self.consume_char());
        }

        return result;
    }


    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(char::is_whitespace)
    }

}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '-' | '_' => true,
        _ => false
    }
}


