/// regex generation rules
/// char = a-z, A-Z, 0-9, "."
///
/// backslash = \d, \D, \s, \S
///
/// [ item - item ] = [char-char]
///
/// item = char | backslash | [ item - item ] | "("items")"
///
/// rep = item (join) "{" num ","? num? "}"
///     | item (join) +
///     | item (join) *
///     | item (join) ?
///
/// ast = item (join) "*" 
///
/// items 
///     = item
///     | rep
///     | ast
///     | items (join) item
///     | items (join) rep
///     | items (join) ast
///
/// or = items "|" items
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    /// \d
    SmallD,
    /// \D
    LargeD,

    /// \s
    SmallS,
    /// \S
    LargeS,

    /// 0-9
    Digit(usize),
    /// a-z, A-Z
    Char(char),

    /// \+
    Plus,
    /// +
    OneOrMore,

    /// \.
    Dot,
    /// .

    Any,
    /// \*
    Ast,
    /// *
    SomeTime,

    /// \|
    Pipe,
    /// |
    Or,

    /// \?
    Question,
    /// ?
    ZeroOrOne,

    /// \(
    BracketRInner,
    /// (
    BracketR,

    /// \)
    BracketLInner,
    /// )
    BracketL,

    /// \{
    CurryRInner,
    /// {
    CurryR,

    /// \}
    CurryLInner,
    /// }
    CurryL,

    /// \[
    SquareRInner,
    /// [
    SquareR,

    /// \]
    SquareLInner,
    /// ]
    SquareL,

    /// \
    BackSlash,
}

pub struct Regex {
    string: String
}

impl Regex {
    pub fn new(string: String) -> Self {
        Regex { string }
    }

    fn tokens_iter(&self) -> RegexTokenIter { 
        RegexTokenIter { item: self.string.chars() }
    }
}

fn parse_backslash(char_: Option<char>) -> Item {
    match char_ {
        Some('d') => Item::SmallD,
        Some('D') => Item::LargeD,
        Some('s') => Item::SmallS,
        Some('S') => Item::LargeS,
        Some('.') => Item::Dot,
        Some('*') => Item::Ast,
        Some('|') => Item::Pipe,
        Some('?') => Item::Question,
        Some('(') => Item::BracketRInner,
        Some(')') => Item::BracketLInner,
        Some('{') => Item::CurryRInner,
        Some('}') => Item::CurryLInner,
        Some('[') => Item::SquareRInner,
        Some(']') => Item::SquareLInner,
        Some('\\') => Item::BackSlash,
        Some(x) => panic!("{}", format!("{} does not follow a backslash", x)),
        None => panic!("backslash cannot end a regular expression.")
    }
}

fn try_digit(char: char) -> Option<Item> {
    char.to_digit(10).map(|x| Item::Digit(x as usize))
}

fn try_special_char(char: char) -> Option<Item> {
    match char {
        '.' => Some(Item::Any),
        '*' => Some(Item::SomeTime),
        '+' => Some(Item::OneOrMore),
        '|' => Some(Item::Or),
        '?' => Some(Item::ZeroOrOne),
        '(' => Some(Item::BracketR),
        ')' => Some(Item::BracketL),
        '{' => Some(Item::CurryR),
        '}' => Some(Item::CurryL),
        '[' => Some(Item::SquareR),
        ']' => Some(Item::SquareL),
        _ => None,
    }
}

pub struct RegexTokenIter<'a> {
    item: Chars<'a>
}

impl Iterator for RegexTokenIter<'_> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.item.next() {
            None => None,
            Some('\\') => Some(parse_backslash(self.item.next())),
            Some(x) => {
                if let Some(item) = try_special_char(x) { return Some(item) }
                else if let Some(item) = try_digit(x) { return Some(item) }
                else { return Some(Item::Char(x)) }
            }
        }
    }
}

#[test]
fn test_parse() {
    let regex_string = "go+gle".to_string();
    let regex = Regex::new(regex_string);
    let mut regex_iter = regex.tokens_iter();
    assert_eq!(Item::Char('g'), regex_iter.next().unwrap());
    assert_eq!(Item::Char('o'), regex_iter.next().unwrap());
    assert_eq!(Item::OneOrMore, regex_iter.next().unwrap());
    assert_eq!(Item::Char('g'), regex_iter.next().unwrap());
    assert_eq!(Item::Char('l'), regex_iter.next().unwrap());
    assert_eq!(Item::Char('e'), regex_iter.next().unwrap());
    assert_eq!(None, regex_iter.next());
}

#[test]
fn test_baskslash() {
    let regex_string = r"\d".to_string();
    let regex = Regex::new(regex_string);
    let mut regex_iter = regex.tokens_iter();
    assert_eq!(Item::SmallD, regex_iter.next().unwrap());
}

#[test]
fn test_escaped_backslash() {
    let regex_string = r"\.".to_string();
    let regex = Regex::new(regex_string);
    let mut regex_iter = regex.tokens_iter();
    assert_eq!(Item::Dot, regex_iter.next().unwrap());
}
