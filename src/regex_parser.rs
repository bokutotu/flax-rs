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
///     | item
///
/// items
///     = rep
///     | items (join) rep
///
/// or = items "|" items
use crate::nfa::Nfa;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Item {
    /// \d
    SmallD,
    /// \D
    LargeD,

    // /// \s
    // SmallS,
    // /// \S
    // LargeS,
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
    BracketLInner,
    /// (
    BracketL,

    /// \)
    BracketRInner,
    /// )
    BracketR,

    /// \{
    CurryLInner,
    /// {
    CurryL,

    /// \}
    CurryRInner,
    /// }
    CurryR,

    /// \[
    SquareLInner,
    /// [
    SquareL,

    /// \]
    SquareRInner,
    /// ]
    SquareR,

    /// \
    BackSlash,
}

// impl Item {
//     fn is_digit(&self) -> bool {
//         matches!(self, Item::Digit(_))
//     }
//
//     fn digit(&self) -> Option<usize> {
//         match self {
//             Item::Digit(x) => Some(*x),
//             _ => None,
//         }
//     }
//
//     fn inner_char(&self) -> Option<char> {
//         match self {
//             Item::Char(x) => Some(*x),
//             _ => None,
//         }
//     }
//
//     fn is_char_match(&self, match_char: char) -> bool {
//         match self.inner_char() {
//             Some(x) => x == match_char,
//             _ => false,
//         }
//     }
// }

fn is_digit(char: &char) -> bool {
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(char)
}

impl PartialEq<char> for Item {
    fn eq(&self, other: &char) -> bool {
        match *self {
            Item::SmallD => is_digit(other),
            Item::LargeD => !is_digit(other),
            Item::Digit(digit) => char::from_digit(digit as u32, 10).unwrap() == *other,
            Item::Char(char_) => char_ == *other,
            Item::Plus => *other == '+',
            Item::Dot => *other == '.',
            Item::Ast => *other == '*',
            Item::Pipe => *other == '|',
            Item::Question => *other == '?',
            Item::BracketRInner => *other == '(',
            Item::BracketLInner => *other == ')',
            Item::CurryRInner => *other == '{',
            Item::CurryLInner => *other == '}',
            Item::SquareRInner => *other == '[',
            Item::SquareLInner => *other == ']',
            Item::BackSlash => *other == '\\',
            _ => unreachable!(),
        }
    }
}

pub struct Regex {
    string: String,
}

impl Regex {
    pub fn new(string: String) -> Self {
        Regex { string }
    }

    pub fn tokens_iter(&self) -> RegexTokenIter {
        RegexTokenIter {
            item: self.string.chars().collect(),
            idx: 0,
        }
    }
}

fn parse_backslash(char_: Option<char>) -> Item {
    match char_ {
        Some('d') => Item::SmallD,
        Some('D') => Item::LargeD,
        // Some('s') => Item::SmallS,
        // Some('S') => Item::LargeS,
        Some('.') => Item::Dot,
        Some('*') => Item::Ast,
        Some('|') => Item::Pipe,
        Some('?') => Item::Question,
        Some('(') => Item::BracketLInner,
        Some(')') => Item::BracketRInner,
        Some('{') => Item::CurryLInner,
        Some('}') => Item::CurryRInner,
        Some('[') => Item::SquareLInner,
        Some(']') => Item::SquareRInner,
        Some('\\') => Item::BackSlash,
        Some(x) => panic!("{}", format!("{} does not follow a backslash", x)),
        None => panic!("backslash cannot end a regular expression."),
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
        '(' => Some(Item::BracketL),
        ')' => Some(Item::BracketR),
        '{' => Some(Item::CurryL),
        '}' => Some(Item::CurryR),
        '[' => Some(Item::SquareL),
        ']' => Some(Item::SquareR),
        _ => None,
    }
}

pub struct RegexTokenIter {
    item: Vec<char>,
    idx: usize,
}

impl RegexTokenIter {
    fn next_char(&mut self) -> Option<char> {
        if self.idx == self.item.len() {
            return None;
        }
        let res = Some(self.item[self.idx]);
        self.idx += 1;
        res
    }

    fn back(&mut self) {
        self.idx -= 1;
    }
}

impl Iterator for RegexTokenIter {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_char() {
            None => None,
            Some('\\') => Some(parse_backslash(self.next_char())),
            Some(x) => {
                if let Some(item) = try_special_char(x) {
                    Some(item)
                } else if let Some(item) = try_digit(x) {
                    Some(item)
                } else {
                    Some(Item::Char(x))
                }
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
fn test_back() {
    let regex_string = "go+gle".to_string();
    let regex = Regex::new(regex_string);
    let mut regex_iter = regex.tokens_iter();

    assert_eq!(Item::Char('g'), regex_iter.next().unwrap());
    assert_eq!(Item::Char('o'), regex_iter.next().unwrap());

    regex_iter.back();

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

pub fn items<T: Clone>(iter: &mut RegexTokenIter) -> Option<Nfa<Item, T>> {
    let mut first = rep(iter)?;
    match items(iter) {
        Some(second) => {
            let first_len = first.len();
            first.concat(first_len, second);
            Some(first)
        }
        None => Some(first),
    }
}

#[derive(Debug, PartialEq)]
struct RepConfig {
    min: usize,
    max: Option<usize>,
}

impl RepConfig {
    fn new(min: usize, max: Option<usize>) -> Self {
        Self { min, max }
    }

    /// This function generates an NFA from the contents of RepConfig.
    // Example
    // { 2, 3 }
    //                       rep_start_idx  rep_end_idx
    //                             |             |
    //                             |             |
    //                            \/             \/
    // . --- NFA --- . --- NFA --- . --- NFA --- .
    //                             |            /\
    //                             |             |
    //                             |             |
    //                             ---------------
    //                                   ε
    // { 2,  }  rep_end_idx  rep_start_idx
    // . --- NFA --- . --- NFA --- .
    //              /\             |
    //               |             |
    //               |             |
    //               ---------------
    //                      ε
    // {2}
    // . --- NFA --- . --- NFA --- .
    //
    // number of concat nfas is max is Some -> max None -> min
    // rep_start_idx -> min * nfa.len()
    // rep_end_idx -> Some(x) -> x * nfa.len() None -> (min - 1) * nfa.len()
    fn nfa<C: Clone, T: Clone>(self, nfa: Nfa<C, T>) -> Nfa<C, T> {
        let base_len = nfa.len();
        let min = self.min;
        let max = match self.max {
            Some(x) => x,
            None => min - 1,
        };
        let num_nfa = usize::max(min, max);
        let mut nfa = nfa;
        nfa.concat_tail_n_time(nfa.clone(), num_nfa);
        if min == max {
            return nfa;
        }
        let rep_start_idx = min * base_len;
        let rep_end_idx = max * base_len;
        nfa[rep_start_idx].add_epsilon(rep_end_idx);
        nfa
    }
}

fn parse_rep(iter: &mut RegexTokenIter) -> Option<RepConfig> {
    match iter.next() {
        None => {
            iter.back();
            None
        }
        // +
        Some(Item::OneOrMore) => Some(RepConfig::new(1, None)),
        // *
        Some(Item::SomeTime) => Some(RepConfig::new(0, None)),
        // ?
        Some(Item::ZeroOrOne) => Some(RepConfig::new(0, Some(1))),
        // "{" min ","? max? "}"
        Some(Item::CurryL) => {
            let min = match iter.next() {
                Some(Item::Digit(x)) => x,
                _ => panic!(),
            };
            match iter.next() {
                None => panic!("何かがおかしいぞい"),
                // "{" min "}"
                Some(Item::CurryR) => Some(RepConfig::new(min, Some(min))),
                // "{" min "," max? "}"
                Some(Item::Char(',')) => {
                    match iter.next() {
                        None => panic!(),
                        // "{" min "," max "}"
                        Some(Item::Digit(max)) => {
                            if matches!(iter.next().unwrap(), Item::CurryR) {
                                // when rep config is { 5, 4 } panic!
                                if max <= min {
                                    panic!();
                                }
                                Some(RepConfig::new(min, Some(max)))
                            } else {
                                panic!();
                            }
                        }
                        // "{" min "," "}"
                        Some(Item::CurryR) => Some(RepConfig::new(min, None)),
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }
        _ => {
            iter.back();
            None
        }
    }
}

fn rep<T: Clone>(iter: &mut RegexTokenIter) -> Option<Nfa<Item, T>> {
    let item = item(iter)?;
    let rep = parse_rep(iter)?;
    Some(rep.nfa(item))
}

fn item<T: Clone>(iter: &mut RegexTokenIter) -> Option<Nfa<Item, T>> {
    match iter.next() {
        None | Some(Item::CurryR) => None,
        Some(Item::CurryL) => {
            let items = items(iter);
            match iter.next() {
                Some(Item::CurryR) => items,
                _ => unreachable!(),
            }
        }
        Some(x) => Some(Nfa::<Item, T>::from_content(x)),
    }
}

#[test]
fn test_rep_config() {
    let regex = Regex::new("*".to_string());
    let mut regex_iter = regex.tokens_iter();
    let rep_config = parse_rep(&mut regex_iter).unwrap();
    assert_eq!(RepConfig::new(0, None), rep_config);

    let regex = Regex::new("?".to_string());
    let mut regex_iter = regex.tokens_iter();
    let rep_config = parse_rep(&mut regex_iter).unwrap();
    assert_eq!(RepConfig::new(0, Some(1)), rep_config);

    let regex = Regex::new("+".to_string());
    let mut regex_iter = regex.tokens_iter();
    let rep_config = parse_rep(&mut regex_iter).unwrap();
    assert_eq!(RepConfig::new(1, None), rep_config);

    let regex = Regex::new("{2}".to_string());
    let mut regex_iter = regex.tokens_iter();
    let rep_config = parse_rep(&mut regex_iter).unwrap();
    assert_eq!(RepConfig::new(2, Some(2)), rep_config);

    let regex = Regex::new("{2,3}".to_string());
    let mut regex_iter = regex.tokens_iter();
    let rep_config = parse_rep(&mut regex_iter).unwrap();
    assert_eq!(RepConfig::new(2, Some(3)), rep_config);

    let regex = Regex::new("{2,}".to_string());
    let mut regex_iter = regex.tokens_iter();
    let rep_config = parse_rep(&mut regex_iter).unwrap();
    assert_eq!(RepConfig::new(2, None), rep_config);
}

#[test]
fn test_item() {
    use crate::nfa::search;
    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    enum TestTerminal {
        A,
    }
    let a = "a".to_string();
    let regex = Regex::new(a);
    let mut regex_iter = regex.tokens_iter();
    let mut nfa = item(&mut regex_iter).unwrap();
    nfa.set_termial_to_last_node(TestTerminal::A);
    let res = search(&nfa, "a");
    assert_eq!(vec![TestTerminal::A], res);
}

#[test]
fn test_rep() {
    use crate::nfa::search;
    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    struct TermianalMarker;
    let regex_string = "a{2,5}".to_string();
    let regex = Regex::new(regex_string);
    let mut regex_iter = regex.tokens_iter();
    let mut nfa = rep(&mut regex_iter).unwrap();
    nfa.set_termial_to_last_node(TermianalMarker);
    let query_string = "aaaaa";
    let res = search(&nfa, query_string);
    assert_eq!(TermianalMarker, res[0]);
}
