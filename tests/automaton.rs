// to test automaton, I use NFA.
use flex::automaton::*;
use flex::nfa::*;
use flex::regex_parser::Item;

#[derive(Debug, Clone, Copy, PartialEq)]
struct TestTerminal;

impl Terminal for TestTerminal {}

type TestState = NfaState<TestTerminal, Item>;

#[test]
fn from_content() {
    let mut ans = NFA::default();
    let content = TestState::from_content(Item::Char('a'));
    let mut node = NfaNode::default();
    node.add_transition(content, 1);
    ans.push(node);
    ans.push(NfaNode::default());
    let res = NFA::from_content(Item::Char('a'));
    assert_eq!(ans, res);
}

#[test]
fn increment_all_index() {
    let mut ans = NFA::default();
    let mut node1 = NfaNode::from_content(Item::Char('a'), 1);
    node1.add_epsilon(2);
    node1.add_terminal(TestTerminal);
    let mut res = NFA::default();
    res.push(node1.clone());
    let res = res.increment_all_index(2);
    node1.increment_all_index(2);
    ans.push(node1);
    assert_eq!(ans, res);
}

#[test]
fn add_state_idx() {
    let mut res = NFA::default();
    let mut node = NfaNode::default();
    node.add_content(Item::Char('a'), 1);
    node.add_terminal(TestTerminal);
    node.add_epsilon(2);
    res.push(node.clone());
    res.add_state_idx_node(0, TestState::from_content(Item::Char('b')), 3);
    node.add_content(Item::Char('b'), 3);
    let mut ans = NFA::default();
    ans.push(node);
    assert_eq!(ans, res);
}

#[test]
fn add_terminal_idx_node() {
    let mut res = NFA::default();
    let mut node = NfaNode::default();
    node.add_content(Item::Char('a'), 1);
    node.add_terminal(TestTerminal);
    node.add_epsilon(2);
    res.push(node.clone());
    res.add_terminal_idx_node(0, TestTerminal);
    node.add_terminal(TestTerminal);
    let mut ans = NFA::default();
    ans.push(node);
    assert_eq!(ans, res);
}

#[test]
fn add_content_idx_node() {
    let mut res = NFA::default();
    let mut node = NfaNode::default();
    node.add_content(Item::Char('a'), 1);
    node.add_terminal(TestTerminal);
    node.add_epsilon(2);
    res.push(node.clone());
    res.add_content_idx_node(0, Item::Char('a'), 3);
    node.add_content(Item::Char('a'), 3);
    let mut ans = NFA::default();
    ans.push(node);
    assert_eq!(ans, res);
}

#[test]
fn set_terminal_idx() {
    let mut res = NFA::default();
    let mut node = NfaNode::default();
    node.add_content(Item::Char('a'), 1);
    node.add_terminal(TestTerminal);
    node.add_epsilon(2);
    res.push(node.clone());
    res.set_terminal_to_idx(0, TestTerminal);
    node.add_terminal(TestTerminal);
    let mut ans = NFA::default();
    ans.push(node);
    assert_eq!(ans, res);
}

#[test]
fn set_terminal_to_last_node() {
    let mut res = NFA::default();
    let mut node1 = NfaNode::default();
    node1.add_content(Item::Char('q'), 1);
    let mut node2 = NfaNode::default();
    res.push(node1.clone());
    res.push(node2.clone());
    res.set_termial_to_last_node(TestTerminal);
    node2.add_terminal(TestTerminal);
    let mut ans = NFA::default();
    ans.push(node1);
    ans.push(node2);
    assert_eq!(ans, res);
}
