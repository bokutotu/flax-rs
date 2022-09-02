use std::cmp::PartialEq;
use std::fmt::Debug;
use std::iter::IntoIterator;

use crate::automaton::{Automaton, Content, NextNode, Node, RegexRun, State, Terminal};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NfaState<T, C> {
    NfaTerminal(T),
    NfaContent(C),
    Epsilon,
}

impl<T, C> State for NfaState<T, C>
where
    T: Terminal,
    C: Content,
{
    type Terminal = T;
    type Content = C;

    fn is_content(&self) -> bool {
        matches!(self, Self::NfaContent(_))
    }

    fn is_terminal(&self) -> bool {
        matches!(self, Self::NfaTerminal(_))
    }

    fn terminal(&self) -> Self::Terminal {
        match self {
            Self::NfaTerminal(x) => *x,
            _ => panic!("this is not terminal"),
        }
    }

    fn content(&self) -> Self::Content {
        match self {
            Self::NfaContent(x) => *x,
            _ => panic!("this is not content"),
        }
    }

    fn from_content(content: Self::Content) -> Self {
        Self::NfaContent(content)
    }

    fn from_terminal(terminal: Self::Terminal) -> Self {
        Self::NfaTerminal(terminal)
    }
}

impl<T: Terminal, C: Content> Default for NfaState<T, C> {
    fn default() -> Self {
        Self::Epsilon
    }
}

impl<T: Terminal, C: Content> NfaState<T, C> {
    pub fn is_epsilon(&self) -> bool {
        matches!(self, Self::Epsilon)
    }

    pub fn from_epsilon() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NfaNode<T: Terminal, C: Content> {
    states: Vec<(NfaState<T, C>, usize)>,
}

impl<T: Terminal, C: Content> Default for NfaNode<T, C> {
    fn default() -> Self {
        Self { states: Vec::new() }
    }
}

impl<T: Terminal, C: Content> Node for NfaNode<T, C> {
    type NodeState = NfaState<T, C>;

    fn add_transition(&mut self, transision: Self::NodeState, idx: usize) {
        self.states.push((transision, idx));
    }

    /// increment index **except** terminal
    fn increment_all_index(&mut self, inc: usize) {
        self.states
            .iter_mut()
            .filter(|(state, _)| !state.is_terminal())
            .for_each(|(_, idx)| *idx += inc);
    }

    fn collect_terminal(&self) -> Vec<T> {
        self.states
            .iter()
            .filter(|(state, _)| state.is_terminal())
            .map(|(terminal, _)| terminal.terminal())
            .collect()
    }

    fn collect_content(&self) -> Vec<(<Self::NodeState as State>::Content, usize)> {
        self.states
            .iter()
            .filter(|(state, _)| state.is_content())
            .map(|(content, idx)| (content.content(), *idx))
            .collect()
    }
}

impl<T: Terminal, C: Content> NfaNode<T, C> {
    fn collect_epsilon_idx(&self) -> Vec<usize> {
        self.states
            .iter()
            .filter(|(state, _)| state.is_epsilon())
            .map(|(_, idx)| *idx)
            .collect()
    }
}

impl<T: Terminal, C: Content> NfaNode<T, C> {
    pub fn add_epsilon(&mut self, idx: usize) {
        let epsion = NfaState::default();
        self.add_transition(epsion, idx);
    }
}

impl<T: Terminal, C: Content> IntoIterator for NfaNode<T, C> {
    type Item = (NfaState<T, C>, usize);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.states.into_iter()
    }
}

pub type NFA<T, C> = Automaton<NfaNode<T, C>>;

impl<T: Terminal, C: Content> NFA<T, C> {
    pub fn add_epsilon_idx_node(&mut self, idx: usize, direction_idx: usize) {
        self[idx].add_epsilon(direction_idx);
    }

    /// Connect another Node's NFA to any NFA node.
    /// 1. add all the idx of the nfa to be connected by the original length
    /// 2. update the length of the original nfa
    /// 3. connect the first index of the nfa to the node to be connected with
    ///    an arbitrary NfaItem
    /// 4. update length of NFA
    pub fn concat(&mut self, source_idx: usize, cat_nfa: NFA<T, C>) {
        let current_len = self.len();
        let cat_nfa = cat_nfa.increment_all_index(current_len - 1);
        self.add_epsilon_idx_node(source_idx, current_len);
        self.append_vec(cat_nfa);
    }

    pub fn concat_tail(&mut self, cat_nfa: NFA<T, C>) {
        let current_len = self.len();
        self.concat(current_len, cat_nfa);
    }

    pub fn concat_tail_n_times(&mut self, cat_nfa: NFA<T, C>, times: usize) {
        for _ in 0..times {
            self.concat_tail(cat_nfa.clone());
        }
    }
}

impl<T: Terminal, C: Content> NextNode for NFA<T, C> {
    type InputState = NfaState<T, C>;
    fn next_node(&self, idx: usize, char_: char) -> Vec<usize> {
        let mut res = Vec::new();
        let mut content_vec = self[idx].collect_char_content_idx(char_);
        self[idx]
            .collect_epsilon_idx()
            .iter()
            .for_each(|epsilon_idx| {
                let mut tmp = self.next_node(*epsilon_idx, char_);
                res.append(&mut tmp);
            });
        res.append(&mut content_vec);
        res
    }
}

impl<T: Terminal, C: Content> RegexRun<NfaNode<T, C>> for NFA<T, C> {}

#[allow(unused_macros)]
macro_rules! mock_struct {
    () => {
        use crate::regex_parser::Item;
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct TestTerminal;
        impl Terminal for TestTerminal {}
        #[allow(dead_code)]
        type NfaTestState = NfaState<TestTerminal, Item>;
    };
}

macro_rules! test_state {
    ($test_fn_name: ident, $state: expr, $test_method: ident, $assert_value: expr, $($sharp: ident)*) => {
        $(
            #[$sharp]
         )*
        #[test]
        fn $test_fn_name() {
            mock_struct!();
            let state: NfaTestState = $state;
            assert_eq!(state.$test_method(), $assert_value);
        }
    };
}

test_state!(
    is_terminal_true,
    NfaTestState::NfaTerminal(TestTerminal),
    is_terminal,
    true,
);
test_state!(
    is_terminal_false1,
    NfaTestState::Epsilon,
    is_terminal,
    false,
);
test_state!(
    is_terminal_false2,
    NfaTestState::NfaContent(Item::Char('a')),
    is_terminal,
    false,
);

test_state!(
    is_content_false2,
    NfaTestState::NfaTerminal(TestTerminal),
    is_content,
    false,
);
test_state!(is_content_false1, NfaTestState::Epsilon, is_content, false,);
test_state!(
    is_content_true,
    NfaTestState::NfaContent(Item::Char('a')),
    is_content,
    true,
);

test_state!(
    is_epsilon_false2,
    NfaTestState::NfaTerminal(TestTerminal),
    is_epsilon,
    false,
);
test_state!(is_epsilon_true, NfaTestState::Epsilon, is_epsilon, true,);
test_state!(
    is_epsilon_false1,
    NfaTestState::NfaContent(Item::Char('a')),
    is_epsilon,
    false,
);

test_state!(
    test_terminal,
    NfaState::NfaTerminal(TestTerminal),
    terminal,
    TestTerminal,
);
test_state!(
    test_terminal_should_panic,
    NfaState::Epsilon,
    terminal,
    TestTerminal,
    should_panic
);
test_state!(
    test_terminal_should_panic2,
    NfaState::NfaContent(Item::Char('a')),
    terminal,
    TestTerminal,
    should_panic
);

test_state!(
    test_content_shoud_panic2,
    NfaState::NfaTerminal(TestTerminal),
    content,
    Item::Char('a'),
    should_panic
);
test_state!(
    test_content_should_panic,
    NfaState::Epsilon,
    content,
    Item::Char('a'),
    should_panic
);
test_state!(
    test_content,
    NfaState::NfaContent(Item::Char('a')),
    content,
    Item::Char('a'),
);

macro_rules! test_state_from {
    ($test_fn_name: ident, $method:ident, $assert_value: expr, $($method_args: expr)*) => {
        #[test]
            fn $test_fn_name() {
                mock_struct!();
                assert_eq!(NfaTestState::$method($($method_args)*), $assert_value);
            }
    }
}

test_state_from!(
    test_from_content,
    from_content,
    NfaTestState::NfaContent(Item::Char('a')),
    Item::Char('a')
);
test_state_from!(
    test_from_terminal,
    from_terminal,
    NfaTestState::NfaTerminal(TestTerminal),
    TestTerminal
);
test_state_from!(test_from_epsilon, from_epsilon, NfaTestState::Epsilon,);

// Test For NfaNode

#[test]
fn node_add_translation() {
    mock_struct!();
    let mut node = NfaNode::default();
    let push_state = NfaTestState::Epsilon;
    node.add_transition(push_state, 1);
    assert_eq!(
        node,
        NfaNode {
            states: vec![(NfaTestState::Epsilon, 1)]
        }
    );
}

#[test]
fn node_add_content() {
    mock_struct!();
    let mut node = NfaNode::default();
    node.add_content(Item::Char('a'), 1);
    assert_eq!(
        node,
        NfaNode {
            states: vec![(NfaTestState::from_content(Item::Char('a')), 1)]
        }
    );
}

#[test]
fn node_add_terminal() {
    mock_struct!();
    let mut node = NfaNode::default();
    node.add_terminal(TestTerminal);
    assert_eq!(
        node,
        NfaNode {
            states: vec![(NfaTestState::from_terminal(TestTerminal), 0)]
        }
    );
}

#[test]
fn node_from_content() {
    mock_struct!();
    let node = NfaNode::from_content(Item::Char('a'), 1);
    assert_eq!(
        node,
        NfaNode {
            states: vec![(NfaTestState::from_content(Item::Char('a')), 1)]
        }
    );
}

#[test]
fn node_add_epsilon() {
    mock_struct!();
    let mut node = NfaNode::default();
    node.add_epsilon(1);
    assert_eq!(
        node,
        NfaNode {
            states: vec![(NfaTestState::from_epsilon(), 1)]
        }
    );
}

#[test]
fn node_increment_all_index() {
    mock_struct!();
    let mut node = NfaNode::default();
    node.add_transition(NfaTestState::Epsilon, 1);
    node.add_transition(NfaTestState::from_content(Item::Char('a')), 2);
    node.add_transition(NfaTestState::from_terminal(TestTerminal), 0);
    node.increment_all_index(2);
    let ans = NfaNode {
        states: vec![
            (NfaTestState::Epsilon, 3),
            (NfaTestState::NfaContent(Item::Char('a')), 4),
            (NfaTestState::NfaTerminal(TestTerminal), 0),
        ],
    };
    assert_eq!(ans, node);
}

macro_rules! node_collect_test {
    ($test_fn_name: ident, $test_method:ident, $ans_vec: expr, $($add_transition: expr),*,, $($method_arg: expr)*) => {
        #[test]
        fn $test_fn_name() {
            mock_struct!();
            let mut node = NfaNode::default();
            $(
                node.add_transition($add_transition, 0);
             )*
            let res = node.$test_method($($method_arg)*);
            assert_eq!(res, $ans_vec);
        }
    };
}

node_collect_test!(
    node_collect_terminal,
    collect_terminal,
    vec![TestTerminal],
    NfaTestState::NfaTerminal(TestTerminal),
    NfaTestState::NfaContent(Item::Char('a')),
    NfaTestState::Epsilon,,
);

node_collect_test!(
    node_collect_terminal_null,
    collect_terminal,
    vec![],
    NfaTestState::NfaContent(Item::Char('a')),
    NfaTestState::Epsilon,,
);

node_collect_test!(
    node_collect_content,
    collect_content,
    vec![(Item::Char('a'), 0), (Item::Char('b'), 0)],
    NfaTestState::NfaContent(Item::Char('a')),
    NfaTestState::NfaContent(Item::Char('b')),
    NfaTestState::Epsilon,,
);

node_collect_test!(
    node_collect_content_null,
    collect_content,
    vec![],
    NfaTestState::Epsilon,
    NfaTestState::NfaTerminal(TestTerminal),
    NfaTestState::Epsilon,,
);

node_collect_test!(
    node_collect_content_idx,
    collect_char_content_idx ,
    vec![0,],
    NfaTestState::NfaContent(Item::Char('a')),
    NfaTestState::NfaContent(Item::Char('b')),
    NfaTestState::Epsilon,,
    'a'
);

node_collect_test!(
    node_collect_content_idx_no_match_char,
    collect_char_content_idx ,
    vec![],
    NfaTestState::NfaContent(Item::Char('a')),
    NfaTestState::NfaContent(Item::Char('b')),
    NfaTestState::Epsilon,,
   'c'
);

node_collect_test!(
    node_collect_content_idx_no_content,
    collect_char_content_idx ,
    vec![],
    NfaTestState::Epsilon,,
   'c'
);

// Test For Automaton
#[test]
fn automaton_from_content() {
    mock_struct!();
    let node_1 = NfaNode::<TestTerminal, Item>::from_content(Item::Char('a'), 1);
    let mut nfa = NFA::default();
    nfa.push(node_1);
    nfa.push(NfaNode::default());
    let ans = NFA::from_content(Item::Char('a'));
    assert_eq!(ans, nfa);
}

#[test]
fn concat() {
    mock_struct!();
    let content_a = NfaNode::<TestTerminal, Item>::from_content(Item::Char('a'), 1);
    let mut epsilon = NfaNode::<TestTerminal, Item>::default();
    epsilon.add_epsilon(2);
    let terminal = NfaNode::<TestTerminal, Item>::from_terminal(TestTerminal);
    let mut nfa_1 = NFA::from_content(Item::Char('a'));
    let mut nfa_2 = NFA::new();
    nfa_2.push(terminal.clone());
    nfa_1.concat(1, nfa_2);
    let mut ans = NFA::new();
    ans.push(content_a);
    ans.push(epsilon);
    ans.push(terminal);

    assert_eq!(ans, nfa_1)
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
// pub enum NfaItem<C, T> {
//     Epsilon,
//     Content(C),
//     Terminal(T),
// }
//
// impl<C, T> NfaItem<C, T> {
//     fn is_epsilon(&self) -> bool {
//         matches!(self, NfaItem::Epsilon)
//     }
//
//     fn is_terminal(&self) -> bool {
//         matches!(self, NfaItem::Terminal(_))
//     }
//
//     fn from_content(content: C) -> Self {
//         NfaItem::Content(content)
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct NfaNode<C, T> {
//     directions: Vec<(NfaItem<C, T>, usize)>,
// }
//
// impl<C, T: Clone> NfaNode<C, T> {
//     fn new() -> Self {
//         Self {
//             directions: Vec::new(),
//         }
//     }
//     fn add_direction(&mut self, dir_char: NfaItem<C, T>, idx: usize) {
//         self.directions.push((dir_char, idx));
//     }
//
//     pub fn add_content(&mut self, content: C, idx: usize) {
//         let item = NfaItem::Content(content);
//         self.add_direction(item, idx);
//     }
//
//     pub fn add_terminal(&mut self, terminal: T) {
//         let item = NfaItem::Terminal(terminal);
//         self.add_direction(item, 0);
//     }
//
//     pub fn add_epsilon(&mut self, idx: usize) {
//         self.add_direction(NfaItem::Epsilon, idx);
//     }
//
//     fn increment_all_index(&mut self, inc: usize) {
//         self.directions.iter_mut().for_each(|(_, idx)| *idx += inc);
//     }
//
//     fn from_content(content: C) -> Self {
//         let item = NfaItem::from_content(content);
//         NfaNode {
//             directions: vec![(item, 1)],
//         }
//     }
// }
//
// #[derive(Clone, PartialEq, Eq, Debug)]
// pub struct Nfa<C, T> {
//     nfa: Vec<NfaNode<C, T>>,
//     len: usize,
// }
//
// impl<C: Clone, T: Clone> Nfa<C, T> {
//     pub fn new() -> Self {
//         Nfa {
//             nfa: Vec::new(),
//             len: 0,
//         }
//     }
//
//     pub fn from_content(content: C) -> Self {
//         let node = NfaNode::from_content(content);
//         let null_node = NfaNode::new();
//         Nfa {
//             nfa: vec![node, null_node],
//             len: 2,
//         }
//     }
//
//     pub fn push(&mut self, node: NfaNode<C, T>) {
//         self.nfa.push(node);
//     }
//
//     fn increment_all_node_idx(self, inc: usize) -> Nfa<C, T> {
//         let mut new = self;
//         new.nfa
//             .iter_mut()
//             .for_each(|node| node.increment_all_index(inc));
//         new
//     }
//
//     fn clone_vec(&self) -> Vec<NfaNode<C, T>> {
//         self.nfa.clone()
//     }
//
//     fn update_len(&mut self) {
//         self.len = self.nfa.len();
//     }
//
//     pub(crate) fn len(&self) -> usize {
//         self.nfa.len()
//     }
//
//     /// Connect another Node's NFA to any NFA node.
//     /// 1. add all the idx of the nfa to be connected by the original length
//     /// 2. update the length of the original nfa
//     /// 3. connect the first index of the nfa to the node to be connected with
//     ///    an arbitrary NfaItem
//     /// 4. update length of NFA
//     pub fn concat(&mut self, source_idx: usize, cat_nfa: Nfa<C, T>) {
//         let cat_nfa = cat_nfa.increment_all_node_idx(self.len());
//         let mut cat_nfa_vec = cat_nfa.clone_vec();
//         self.nfa[source_idx].add_epsilon(self.len);
//         self.nfa.append(&mut cat_nfa_vec);
//         self.update_len();
//     }
//
//     pub fn concat_tail(&mut self, cat_nfa: Nfa<C, T>) {
//         let len = self.len();
//         self.concat(len - 1, cat_nfa);
//     }
//
//     pub fn concat_tail_n_time(&mut self, cat_nfa: Nfa<C, T>, times: usize) {
//         for _ in 0..times {
//             self.concat_tail(cat_nfa.clone());
//         }
//     }
//
//     pub fn set_termial_to_idx(&mut self, idx: usize, terminal: T) {
//         self.index_mut(idx).add_terminal(terminal);
//     }
//
//     pub fn set_termial_to_last_node(&mut self, terminal: T) {
//         let last_idx = self.len() - 1;
//         self.set_termial_to_idx(last_idx, terminal);
//     }
// }
//
// impl<C: Clone, T: Clone> Default for Nfa<C, T> {
//     fn default() -> Self {
//         Self::new()
//     }
// }
//
// fn search_inner<C, T>(nfa: &Nfa<C, T>, search_char_vec: &[char], idx: usize) -> Vec<T>
// where
//     C: PartialEq<char> + Clone + Debug,
//     T: Clone + Copy + Debug,
// {
//     let mut res = Vec::new();
//     if search_char_vec.is_empty() {
//         for (item, idx) in nfa[idx]
//             .directions
//             .iter()
//             .filter(|(item, _)| item.is_terminal() | item.is_epsilon())
//         {
//             match item {
//                 NfaItem::Terminal(x) => res.push(*x),
//                 NfaItem::Epsilon => {
//                     let mut tmp = search_inner(nfa, search_char_vec, *idx);
//                     res.append(&mut tmp);
//                 }
//                 _ => unreachable!(),
//             }
//         }
//     } else {
//         let tmp_char = search_char_vec[0];
//         let next_search_char_vec = &search_char_vec[1..];
//
//         nfa[idx]
//             .directions
//             .iter()
//             .for_each(|(item, idx)| match item {
//                 NfaItem::Terminal(x) => res.push(*x),
//                 NfaItem::Epsilon => {
//                     let mut tmp = search_inner(nfa, search_char_vec, *idx);
//                     res.append(&mut tmp);
//                 }
//                 NfaItem::Content(x) => {
//                     if *x == tmp_char {
//                         let mut tmp = search_inner(nfa, next_search_char_vec, *idx);
//                         res.append(&mut tmp);
//                     }
//                 }
//             });
//     }
//     res
// }
//
// pub fn search<C, T>(nfa: &Nfa<C, T>, search_string: &str) -> Vec<T>
// where
//     C: PartialEq<char> + Clone + Debug,
//     T: Copy + Debug,
// {
//     let search_char_vec: Vec<char> = search_string.chars().collect::<Vec<_>>();
//     search_inner(nfa, &search_char_vec, 0)
// }
//
// impl<C, T> Index<usize> for Nfa<C, T> {
//     type Output = NfaNode<C, T>;
//
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.nfa[index]
//     }
// }
//
// impl<C, T> IndexMut<usize> for Nfa<C, T> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.nfa[index]
//     }
// }
//
// impl<'a, C, T> Index<usize> for &'a Nfa<C, T> {
//     type Output = NfaNode<C, T>;
//
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.nfa[index]
//     }
// }
//
// impl<'a, C, T> Index<usize> for &'a mut Nfa<C, T> {
//     type Output = NfaNode<C, T>;
//
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.nfa[index]
//     }
// }
//
// impl<'a, C, T> IndexMut<usize> for &'a mut Nfa<C, T> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.nfa[index]
//     }
// }
//
// #[cfg(test)]
// mod nfa_trasition_test {
//     use crate::nfa::{search, Nfa, NfaItem, NfaNode};
//
//     impl<C, T> Nfa<C, T> {
//         fn from_vec(nfa: Vec<NfaNode<C, T>>) -> Self {
//             let len = nfa.len();
//             Self { nfa, len }
//         }
//     }
//
//     #[derive(Clone, Copy, Debug, PartialEq, Eq)]
//     enum Terminal {
//         Rust,
//         Zig,
//         Ruby,
//         CXX,
//     }
//
//     fn construct_nfa() -> Nfa<char, Terminal> {
//         // rust nfa
//         let first = NfaNode {
//             directions: vec![(NfaItem::Epsilon, 1)],
//         };
//         let r = NfaNode {
//             directions: vec![(NfaItem::Content('r'), 2)],
//         };
//         let u = NfaNode {
//             directions: vec![(NfaItem::Content('u'), 3)],
//         };
//         let s = NfaNode {
//             directions: vec![(NfaItem::Content('s'), 4)],
//         };
//         let t = NfaNode {
//             directions: vec![(NfaItem::Content('t'), 5)],
//         };
//         let terminal = NfaNode {
//             directions: vec![(NfaItem::Terminal(Terminal::Rust), 0)],
//         };
//         let mut rust_nfa = Nfa::from_vec(vec![first, r, u, s, t, terminal]);
//
//         // ruby nfa
//         let r = NfaNode {
//             directions: vec![(NfaItem::Content('r'), 1)],
//         };
//         let u = NfaNode {
//             directions: vec![(NfaItem::Content('u'), 2)],
//         };
//         let b = NfaNode {
//             directions: vec![(NfaItem::Content('b'), 3)],
//         };
//         let y = NfaNode {
//             directions: vec![(NfaItem::Content('y'), 4)],
//         };
//         let terminal = NfaNode {
//             directions: vec![(NfaItem::Terminal(Terminal::Ruby), 0)],
//         };
//         let ruby_nfa = Nfa::from_vec(vec![r, u, b, y, terminal]);
//
//         // zig nfa
//         let z = NfaNode {
//             directions: vec![(NfaItem::Content('z'), 1)],
//         };
//         let i = NfaNode {
//             directions: vec![(NfaItem::Content('i'), 2)],
//         };
//         let g = NfaNode {
//             directions: vec![(NfaItem::Content('g'), 3)],
//         };
//         let terminal = NfaNode {
//             directions: vec![(NfaItem::Terminal(Terminal::Zig), 0)],
//         };
//         let zig_nfa = Nfa::from_vec(vec![z, i, g, terminal]);
//
//         // cxx nfa
//         let c = NfaNode {
//             directions: vec![(NfaItem::Content('c'), 1)],
//         };
//         let x_1 = NfaNode {
//             directions: vec![(NfaItem::Content('x'), 2)],
//         };
//         let x_2 = NfaNode {
//             directions: vec![(NfaItem::Content('x'), 3)],
//         };
//         let terminal = NfaNode {
//             directions: vec![(NfaItem::Terminal(Terminal::CXX), 0)],
//         };
//         let cxx_nfa = Nfa::from_vec(vec![c, x_1, x_2, terminal]);
//
//         rust_nfa.concat(0, ruby_nfa);
//         rust_nfa.concat(0, zig_nfa);
//         rust_nfa.concat(0, cxx_nfa);
//
//         rust_nfa
//     }
//
//     #[test]
//     fn test_rust_string() {
//         let nfa = construct_nfa();
//         let res = search::<char, Terminal>(&nfa, "rust");
//         assert_eq!(vec![Terminal::Rust], res);
//     }
//
//     #[test]
//     fn test_ruby_string() {
//         let nfa = construct_nfa();
//         let res = search::<char, Terminal>(&nfa, "ruby");
//         assert_eq!(res, vec![Terminal::Ruby])
//     }
//
//     #[test]
//     fn test_cxx_string() {
//         let nfa = construct_nfa();
//         let res = search(&nfa, "cxx");
//         assert_eq!(res, vec![Terminal::CXX]);
//     }
//
//     #[test]
//     fn test_zig_string() {
//         let nfa = construct_nfa();
//         let res = search(&nfa, "zig");
//         assert_eq!(res, vec![Terminal::Zig]);
//     }
// }
