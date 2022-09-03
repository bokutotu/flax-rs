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

// impl<T: Terminal, C: Content> NfaNode<T, C> {
//     fn collect_epsilon_idx(&self) -> Vec<usize> {
//         self.states
//             .iter()
//             .filter(|(state, _)| state.is_epsilon())
//             .map(|(_, idx)| *idx)
//             .collect()
//     }
// }

impl<T: Terminal, C: Content> NfaNode<T, C> {
    pub fn from_epsilon(idx: usize) -> Self {
        let state = NfaState::from_epsilon();
        let mut default = Self::default();
        default.add_transition(state, idx);
        default
    }

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
        let cat_nfa = cat_nfa.increment_all_index(current_len);
        self.add_epsilon_idx_node(source_idx, current_len);
        self.append_vec(cat_nfa);
    }

    pub fn concat_tail(&mut self, cat_nfa: NFA<T, C>) {
        let current_len = self.len() - 1;
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
        self[idx]
            .clone()
            .into_iter()
            .filter(|(state, _)| !(state.is_content() && state.content() != char_))
            .map(|(_, idx_)| idx_)
            .collect()
    }
}

impl<T: Terminal, C: Content> RegexRun<NfaNode<T, C>> for NFA<T, C> {}

// ----------------------------------
// ----------------------------------
// ----------------------------------
// ----------------------------------
// ----------------------------------
// ----------------------------------
// ----------------------------------
// ----------------------------------
// ----------------------------------
// ----------------------------------
// test
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
    let mut res = NFA::<TestTerminal, Item>::from_content(Item::Char('a'));
    let b = NFA::<TestTerminal, Item>::from_content(Item::Char('b'));
    res.concat(1, b);

    let mut ans = NFA::new();
    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('a'),
        1,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::from_epsilon(2));
    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('b'),
        3,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::default());
    assert_eq!(res, ans);
}

#[test]
fn concat_first_node() {
    mock_struct!();
    let mut res = NFA::new();
    res.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('a'),
        1,
    ));
    res.push(NfaNode::<TestTerminal, Item>::from_epsilon(2));
    res.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('b'),
        3,
    ));
    res.push(NfaNode::<TestTerminal, Item>::from_epsilon(4));
    res.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('c'),
        5,
    ));
    res.push(NfaNode::<TestTerminal, Item>::from_terminal(TestTerminal));
    let concat_nfa = NFA::from_content(Item::Char('d'));
    res.concat(0, concat_nfa);
    let mut first_node = NfaNode::<TestTerminal, Item>::from_content(Item::Char('a'), 1);
    first_node.add_epsilon(6);
    let mut ans = NFA::new();
    ans.push(first_node);
    ans.push(NfaNode::<TestTerminal, Item>::from_epsilon(2));
    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('b'),
        3,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::from_epsilon(4));
    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('c'),
        5,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::from_terminal(TestTerminal));
    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('d'),
        7,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::default());
    assert_eq!(ans, res);
}

#[test]
fn concat_tail() {
    mock_struct!();
    let mut res = NFA::<TestTerminal, Item>::from_content(Item::Char('a'));
    let node_b = NFA::<TestTerminal, Item>::from_content(Item::Char('b'));
    res.concat(1, node_b);
    let condcat_nfa = NFA::<TestTerminal, Item>::from_content(Item::Char('c'));
    res.concat_tail(condcat_nfa.clone());
    res.concat_tail(condcat_nfa.clone());

    let mut ans = NFA::new();
    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('a'),
        1,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::from_epsilon(2));
    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('b'),
        3,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::from_epsilon(4));
    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('c'),
        5,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::from_epsilon(6));
    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('c'),
        7,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::default());
    assert_eq!(ans, res);
}

#[test]
fn concat_tail_n_times() {
    mock_struct!();
    let mut res = NFA::<TestTerminal, Item>::from_content(Item::Char('a'));
    let node_b = NFA::<TestTerminal, Item>::from_content(Item::Char('b'));
    res.concat_tail(node_b);
    let concat_nfa = NFA::<TestTerminal, Item>::from_content(Item::Char('c'));
    res.concat_tail_n_times(concat_nfa, 2);

    let mut ans = NFA::new();

    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('a'),
        1,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::from_epsilon(2));
    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('b'),
        3,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::from_epsilon(4));
    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('c'),
        5,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::from_epsilon(6));

    ans.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('c'),
        7,
    ));
    ans.push(NfaNode::<TestTerminal, Item>::default());

    assert_eq!(ans, res);
}

#[test]
fn next_node_without_epsilon() {
    mock_struct!();
    let mut automaton = NFA::new();
    let mut node = NfaNode::<TestTerminal, Item>::from_content(Item::Char('a'), 1);
    node.add_content(Item::Char('b'), 100);
    node.add_content(Item::Char('a'), 200);
    automaton.push(node);
    let res = automaton.next_node(0, 'a');
    let ans = vec![1, 200];
    assert_eq!(res, ans);
}

#[test]
fn next_node_with_exception() {
    mock_struct!();
    let mut automaton = NFA::new();
    let mut node_0 = NfaNode::<TestTerminal, Item>::from_content(Item::Char('a'), 1);
    node_0.add_epsilon(2);
    let node_1 = NfaNode::<TestTerminal, Item>::default();
    automaton.push(node_0);
    automaton.push(node_1);
    let mut res = automaton.next_node(0, 'a');
    let mut ans = vec![1, 2];
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}

#[test]
fn next_node_epsilon() {
    mock_struct!();
    let mut automaton = NFA::new();
    automaton.push(NfaNode::<TestTerminal, Item>::from_epsilon(1));
    automaton.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('a'),
        2,
    ));
    let res = automaton.next_node(0, 'a');
    let ans = vec![1];
    assert_eq!(res, ans);
}

#[test]
fn next_node_terminal_content() {
    mock_struct!();
    let mut automaton = NFA::new();
    let mut node_0 = NfaNode::from_epsilon(2);
    node_0.add_content(Item::Char('a'), 1);
    let node_1 = NfaNode::default();
    let node_2 = NfaNode::from_terminal(TestTerminal);
    automaton.push(node_0);
    automaton.push(node_1);
    automaton.push(node_2);
    let mut res = automaton.next_node(0, 'a');
    println!("{:?}", automaton);
    let mut ans = vec![1, 2];
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}

#[test]
fn regex_run_strait_automaton() {
    mock_struct!();
    let mut automaton = NFA::new();
    automaton.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('r'),
        1,
    ));
    automaton.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('u'),
        2,
    ));
    automaton.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('s'),
        3,
    ));
    automaton.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('t'),
        4,
    ));
    automaton.push(NfaNode::<TestTerminal, Item>::from_terminal(TestTerminal));
    let string = "rust".to_string();
    let res = automaton.run(&string);
    let ans = vec![TestTerminal];
    assert_eq!(res, ans);
}

#[test]
fn regex_run_many_path() {
    mock_struct!();
    let mut automaton = NFA::new();
    automaton.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('r'),
        1,
    ));
    automaton.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('u'),
        2,
    ));
    automaton.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('s'),
        3,
    ));
    automaton.push(NfaNode::<TestTerminal, Item>::from_content(
        Item::Char('t'),
        4,
    ));
    automaton.push(NfaNode::<TestTerminal, Item>::from_terminal(TestTerminal));
    automaton.add_state_idx_node(0, NfaState::Epsilon, 4);
    automaton.add_state_idx_node(1, NfaState::Epsilon, 4);
    automaton.add_state_idx_node(2, NfaState::Epsilon, 4);
    automaton.add_state_idx_node(3, NfaState::Epsilon, 4);
    println!("{:?}", automaton);
    let string = "rust".to_string();
    let res = automaton.run(&string);
    let ans = vec![
        TestTerminal,
        TestTerminal,
        TestTerminal,
        TestTerminal,
        TestTerminal,
    ];
    assert_eq!(res, ans);
}
