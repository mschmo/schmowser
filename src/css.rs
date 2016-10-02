//! Simple CSS parser
//! CSS 2.1 simple selectors
//! https://www.w3.org/TR/CSS2/selector.html#selector-syntax
//!
//! Does not support @ media queries, comments, and many selectors/values/units

// Stylesheet is just a series of rules
struct Stylesheet {
    rules: Vec<Rule>
}

// Selectors separated by commas, followed by declarations in braces
struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>
}

enum Selector {
    Simple(SimpleSelector)
}

// ID prefixed by '#', class names prefixed by '.'
// If tag_name is empty or '*', then it is a "universal selector"
struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>
}

// Name/Value pair (e.g. "margin: auto;")
struct Declaration {
    name: String,
    value: Value
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color)
}

// TODO: add more unit types
enum Unit {
    Px
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

// Specificity for deciding who takes precedence when styles override each other
pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(ref simple) = *self;
        // Ignore universal selector
        if simple.tag_name == '*' {
            return (0, 0, 0);
        }
        let a = simple.id.iter().count();           // Number of ID selectors
        let b = simple.class.len();                 // Number of class selectors
        let c = simple.tag_name.iter().count();     // Number of type selectors
        (a, b, c)
    }
}

/* Now build the CSS parser */

struct Parser {
    pos: usize,
    input: String
}

impl Parser {
    // parser methods
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new()
        };
        // TODO: Error checking for crap like '###'
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
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break
            }
        }
        return selector;
    }

    // Parse a rule set: &lt;selectors&gt; { &lt;declarations&gt; }.
    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations()
        }
    }

    // Parse comma separated list of selectors
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list!", c)
            }
        }
        // Order by specificity
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        return selectors;
    }


}
