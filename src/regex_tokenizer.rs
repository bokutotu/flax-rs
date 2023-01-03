//! 正規表現のトークナイザー
//! 特殊記号、数字、などを分離してトークンにする


/// トークンの種類を表す
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

// impl Content for Item {}

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
            Item::Any => true,
            _ => unreachable!(),
        }
    }
}

impl PartialEq<Item> for char {
    fn eq(&self, other: &Item) -> bool {
        other == self
    }
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        Item::Char(c)
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

#[derive(Debug)]
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

// pub fn items<T: Terminal>(iter: &mut RegexTokenIter) -> Option<NFA<T, Item>> {
//     let mut first = rep(iter)?;
//     match items(iter) {
//         Some(second) => {
//             let first_len = first.len() - 1;
//             first.concat(first_len, second);
//             Some(first)
//         }
//         None => Some(first),
//     }
// }
//
// #[derive(Debug, PartialEq)]
// struct RepConfig {
//     min: usize,
//     max: Option<usize>,
// }
//
// impl RepConfig {
//     fn new(min: usize, max: Option<usize>) -> Self {
//         Self { min, max }
//     }
//
//     /// This function generates an NFA from the contents of RepConfig.
//     // Example
//     // { 2, 3 }
//     //                       rep_start_idx  rep_end_idx
//     //                             |             |
//     //                             |             |
//     //                            \/             \/
//     // . --- NFA --- . --- NFA --- . --- NFA --- .
//     //                             |            /\
//     //                             |             |
//     //                             |             |
//     //                             ---------------
//     //                                   ε
//     // { 2,  }  rep_end_idx  rep_start_idx
//     // . --- NFA --- . --- NFA --- .
//     //              /\             |
//     //               |             |
//     //               |             |
//     //               ---------------
//     //                      ε
//     // {2}
//     // . --- NFA --- . --- NFA --- .
//     //
//     // number of concat nfas is max is Some -> max None -> min
//     // rep_start_idx -> min * nfa.len()
//     // rep_end_idx -> Some(x) -> x * nfa.len() None -> (min - 1) * nfa.len()
//     fn nfa<T: Terminal, C: Content>(self, nfa: NFA<T, C>) -> NFA<T, C> {
//         let base_len = nfa.len();
//         let min = self.min;
//         let max = match self.max {
//             Some(x) => x,
//             None => min - 1,
//         };
//         let num_nfa = usize::max(min, max);
//         let mut nfa = nfa;
//         nfa.concat_tail_n_times(nfa.clone(), num_nfa - 1);
//         if min == max {
//             return nfa;
//         }
//         let rep_start_idx = min * base_len - 1;
//         let rep_end_idx = max * base_len - 1;
//         (rep_start_idx..rep_end_idx)
//             .step_by(base_len)
//             .for_each(|x| nfa[x].add_epsilon(rep_end_idx));
//         nfa
//     }
// }
//
// fn parse_rep(iter: &mut RegexTokenIter) -> Option<RepConfig> {
//     match iter.next() {
//         None => None,
//         // +
//         Some(Item::OneOrMore) => Some(RepConfig::new(1, None)),
//         // *
//         Some(Item::SomeTime) => Some(RepConfig::new(0, None)),
//         // ?
//         Some(Item::ZeroOrOne) => Some(RepConfig::new(0, Some(1))),
//         // "{" min ","? max? "}"
//         Some(Item::CurryL) => {
//             let min = match iter.next() {
//                 Some(Item::Digit(x)) => x,
//                 _ => panic!(),
//             };
//             match iter.next() {
//                 None => panic!("何かがおかしいぞい"),
//                 // "{" min "}"
//                 Some(Item::CurryR) => Some(RepConfig::new(min, Some(min))),
//                 // "{" min "," max? "}"
//                 Some(Item::Char(',')) => {
//                     match iter.next() {
//                         None => panic!(),
//                         // "{" min "," max "}"
//                         Some(Item::Digit(max)) => {
//                             if matches!(iter.next().unwrap(), Item::CurryR) {
//                                 // when rep config is { 5, 4 } panic!
//                                 if max <= min {
//                                     panic!();
//                                 }
//                                 Some(RepConfig::new(min, Some(max)))
//                             } else {
//                                 panic!();
//                             }
//                         }
//                         // "{" min "," "}"
//                         Some(Item::CurryR) => Some(RepConfig::new(min, None)),
//                         _ => panic!(),
//                     }
//                 }
//                 _ => panic!(),
//             }
//         }
//         _ => {
//             iter.back();
//             None
//         }
//     }
// }
//
// fn rep<T: Terminal>(iter: &mut RegexTokenIter) -> Option<NFA<T, Item>> {
//     let item = item(iter)?;
//     match parse_rep(iter) {
//         Some(rep) => Some(rep.nfa(item)),
//         None => Some(item),
//     }
// }
//
// fn item<T: Terminal>(iter: &mut RegexTokenIter) -> Option<NFA<T, Item>> {
//     println!("{:?}", iter);
//     match iter.next() {
//         None | Some(Item::BracketR) => None,
//         Some(Item::BracketL) => items(iter),
//         Some(x) => Some(NFA::<T, Item>::from_content(x)),
//     }
// }

macro_rules! check_item {
    (@define_item $arm:ident, $($arg:expr)+) => {
        Item::$arm($($arg)+,)
    };

    (@define_item $arm:ident,) => {
        Item::$arm
    };

    (@eq $test_fn_name:ident, $arm:ident, $cmp:expr, $($arg: expr)*) => {
        #[test]
        fn $test_fn_name() {
            assert!(check_item!(@define_item $arm, $($arg)*) == $cmp)
        }
    };

    (@neq $test_fn_name:ident, $arm:ident, $cmp:expr, $($arg: expr)*) => {
        #[test]
        fn $test_fn_name() {
            assert!(check_item!(@define_item $arm, $($arg)*) != $cmp)
        }
    };
}

check_item!(@eq item_char, Char, 'a', 'a');
check_item!(@neq item_char_neq, Char, 'a', 'b');
check_item!(@eq item_digit, Digit, '0', 0);
check_item!(@neq item_digit_neq, Digit, '0', 1);
check_item!(@neq item_digit_char_neq, Digit, 'a', 0);
check_item!(@eq item_any, Any, 'a',);
check_item!(@eq item_smalld, SmallD, '0',);
check_item!(@neq item_smalld_neq, SmallD, 'a',);
check_item!(@eq item_large_d, LargeD, 'a',);
check_item!(@neq item_larged_neq, LargeD, '0',);

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
fn test_rep_regex() {
    let regex_string = "(abc){2,3}".to_string();
    let regex = Regex::new(regex_string);
    let mut regex_iter = regex.tokens_iter();
    assert_eq!(Item::BracketL, regex_iter.next().unwrap());
    assert_eq!(Item::Char('a'), regex_iter.next().unwrap());
    assert_eq!(Item::Char('b'), regex_iter.next().unwrap());
    assert_eq!(Item::Char('c'), regex_iter.next().unwrap());
    assert_eq!(Item::BracketR, regex_iter.next().unwrap());
    assert_eq!(Item::CurryL, regex_iter.next().unwrap());
    assert_eq!(Item::Digit(2), regex_iter.next().unwrap());
    assert_eq!(Item::Char(','), regex_iter.next().unwrap());
    assert_eq!(Item::Digit(3), regex_iter.next().unwrap());
    assert_eq!(Item::CurryR, regex_iter.next().unwrap());
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

// macro_rules! rep_config {
//     ($fn_name:ident, $regex_string:expr, $ans:expr) => {
//         #[test]
//         fn $fn_name() {
//             let regex = Regex::new($regex_string);
//             let mut regex_iter = regex.tokens_iter();
//             let rep_config = parse_rep(&mut regex_iter).unwrap();
//             assert_eq!($ans, rep_config);
//         }
//     };
// }

// rep_config!(rep_config_struct_gen_ast, "*".to_string(), RepConfig::new(0, None));
// rep_config!(rep_config_struct_gen_qus, "?".to_string(), RepConfig::new(0, Some(1)));
// rep_config!(rep_config_struct_gen_plus, "+".to_string(), RepConfig::new(1, None));
// rep_config!(rep_config_struct_gen_num, "{2}".to_string(), RepConfig::new(2, Some(2)));
// rep_config!(rep_config_struct_gen_num_num, "{2,3}".to_string(), RepConfig::new(2, Some(3)));
// rep_config!(rep_config_struct_gen_num_open, "{2,}".to_string(), RepConfig::new(2, None));
