use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum NfaItem<C, T> {
    Epsilon,
    Content(C),
    Terminal(T),
}

impl<C, T> NfaItem<C, T> {
    fn is_epsilon(&self) -> bool {
        matches!(self, NfaItem::Epsilon)
    }
    //
    // fn is_content(&self) -> bool {
    //     matches!(self, NfaItem::Content(_))
    // }

    fn is_terminal(&self) -> bool {
        matches!(self, NfaItem::Terminal(_))
    }

    fn from_content(content: C) -> Self {
        NfaItem::Content(content)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<C, T> {
    directions: Vec<(NfaItem<C, T>, usize)>,
}

impl<C, T: Clone> Node<C, T> {
    fn new() -> Self {
        Self {
            directions: Vec::new(),
        }
    }
    fn add_direction(&mut self, dir_char: NfaItem<C, T>, idx: usize) {
        self.directions.push((dir_char, idx));
    }

    pub fn add_content(&mut self, content: C, idx: usize) {
        let item = NfaItem::Content(content);
        self.add_direction(item, idx);
    }

    pub fn add_terminal(&mut self, terminal: T) {
        let item = NfaItem::Terminal(terminal);
        self.add_direction(item, 0);
    }

    pub fn add_epsilon(&mut self, idx: usize) {
        self.add_direction(NfaItem::Epsilon, idx);
    }

    fn increment_all_index(&mut self, inc: usize) {
        self.directions.iter_mut().for_each(|(_, idx)| *idx += inc);
    }

    fn from_content(content: C) -> Self {
        let item = NfaItem::from_content(content);
        Node {
            directions: vec![(item, 1)],
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Nfa<C, T> {
    nfa: Vec<Node<C, T>>,
    len: usize,
}

impl<C: Clone, T: Clone> Nfa<C, T> {
    pub fn new() -> Self {
        Nfa {
            nfa: Vec::new(),
            len: 0,
        }
    }

    pub fn from_content(content: C) -> Self {
        let node = Node::from_content(content);
        let null_node = Node::new();
        Nfa {
            nfa: vec![node, null_node],
            len: 2,
        }
    }

    pub fn push(&mut self, node: Node<C, T>) {
        self.nfa.push(node);
    }

    fn increment_all_node_idx(self, inc: usize) -> Nfa<C, T> {
        let mut new = self;
        new.nfa
            .iter_mut()
            .for_each(|node| node.increment_all_index(inc));
        new
    }

    fn clone_vec(&self) -> Vec<Node<C, T>> {
        self.nfa.clone()
    }

    fn update_len(&mut self) {
        self.len = self.nfa.len();
    }

    pub(crate) fn len(&self) -> usize {
        self.nfa.len()
    }

    /// Connect another Node's NFA to any NFA node.
    /// 1. add all the idx of the nfa to be connected by the original length
    /// 2. update the length of the original nfa
    /// 3. connect the first index of the nfa to the node to be connected with
    ///    an arbitrary NfaItem
    /// 4. update length of NFA
    pub fn concat(&mut self, source_idx: usize, cat_nfa: Nfa<C, T>) {
        let cat_nfa = cat_nfa.increment_all_node_idx(self.len());
        let mut cat_nfa_vec = cat_nfa.clone_vec();
        self.nfa[source_idx].add_epsilon(self.len);
        self.nfa.append(&mut cat_nfa_vec);
        self.update_len();
    }

    pub fn concat_tail(&mut self, cat_nfa: Nfa<C, T>) {
        let len = self.len();
        self.concat(len - 1, cat_nfa);
    }

    pub fn concat_tail_n_time(&mut self, cat_nfa: Nfa<C, T>, times: usize) {
        for _ in 0..times {
            self.concat_tail(cat_nfa.clone());
        }
    }

    pub fn set_termial_to_idx(&mut self, idx: usize, terminal: T) {
        self.index_mut(idx).add_terminal(terminal);
    }

    pub fn set_termial_to_last_node(&mut self, terminal: T) {
        let last_idx = self.len() - 1;
        self.set_termial_to_idx(last_idx, terminal);
    }
}

impl<C: Clone, T: Clone> Default for Nfa<C, T> {
    fn default() -> Self {
        Self::new()
    }
}

fn search_inner<C, T>(nfa: &Nfa<C, T>, search_char_vec: &[char], idx: usize) -> Vec<T>
where
    C: PartialEq<char> + Clone + Debug,
    T: Clone + Copy + Debug,
{
    let mut res = Vec::new();
    if search_char_vec.is_empty() {
        for (item, idx) in nfa[idx]
            .directions
            .iter()
            .filter(|(item, _)| item.is_terminal() | item.is_epsilon())
        {
            match item {
                NfaItem::Terminal(x) => res.push(*x),
                NfaItem::Epsilon => {
                    let mut tmp = search_inner(nfa, search_char_vec, *idx);
                    res.append(&mut tmp);
                }
                _ => unreachable!(),
            }
        }
    } else {
        let tmp_char = search_char_vec[0];
        let next_search_char_vec = &search_char_vec[1..];

        nfa[idx]
            .directions
            .iter()
            .for_each(|(item, idx)| match item {
                NfaItem::Terminal(x) => res.push(*x),
                NfaItem::Epsilon => {
                    let mut tmp = search_inner(nfa, search_char_vec, *idx);
                    res.append(&mut tmp);
                }
                NfaItem::Content(x) => {
                    if *x == tmp_char {
                        let mut tmp = search_inner(nfa, next_search_char_vec, *idx);
                        res.append(&mut tmp);
                    }
                }
            });
    }
    res
}

pub fn search<C, T>(nfa: &Nfa<C, T>, search_string: &str) -> Vec<T>
where
    C: PartialEq<char> + Clone + Debug,
    T: Copy + Debug,
{
    let search_char_vec: Vec<char> = search_string.chars().collect::<Vec<_>>();
    search_inner(nfa, &search_char_vec, 0)
}

impl<C, T> Index<usize> for Nfa<C, T> {
    type Output = Node<C, T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nfa[index]
    }
}

impl<C, T> IndexMut<usize> for Nfa<C, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nfa[index]
    }
}

impl<'a, C, T> Index<usize> for &'a Nfa<C, T> {
    type Output = Node<C, T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nfa[index]
    }
}

impl<'a, C, T> Index<usize> for &'a mut Nfa<C, T> {
    type Output = Node<C, T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nfa[index]
    }
}

impl<'a, C, T> IndexMut<usize> for &'a mut Nfa<C, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nfa[index]
    }
}

#[cfg(test)]
mod nfa_trasition_test {
    use crate::nfa::{search, Nfa, NfaItem, Node};

    impl<C, T> Nfa<C, T> {
        fn from_vec(nfa: Vec<Node<C, T>>) -> Self {
            let len = nfa.len();
            Self { nfa, len }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Terminal {
        Rust,
        Zig,
        Ruby,
        CXX,
    }

    fn construct_nfa() -> Nfa<char, Terminal> {
        // rust nfa
        let first = Node {
            directions: vec![(NfaItem::Epsilon, 1)],
        };
        let r = Node {
            directions: vec![(NfaItem::Content('r'), 2)],
        };
        let u = Node {
            directions: vec![(NfaItem::Content('u'), 3)],
        };
        let s = Node {
            directions: vec![(NfaItem::Content('s'), 4)],
        };
        let t = Node {
            directions: vec![(NfaItem::Content('t'), 5)],
        };
        let terminal = Node {
            directions: vec![(NfaItem::Terminal(Terminal::Rust), 0)],
        };
        let mut rust_nfa = Nfa::from_vec(vec![first, r, u, s, t, terminal]);

        // ruby nfa
        let r = Node {
            directions: vec![(NfaItem::Content('r'), 1)],
        };
        let u = Node {
            directions: vec![(NfaItem::Content('u'), 2)],
        };
        let b = Node {
            directions: vec![(NfaItem::Content('b'), 3)],
        };
        let y = Node {
            directions: vec![(NfaItem::Content('y'), 4)],
        };
        let terminal = Node {
            directions: vec![(NfaItem::Terminal(Terminal::Ruby), 0)],
        };
        let ruby_nfa = Nfa::from_vec(vec![r, u, b, y, terminal]);

        // zig nfa
        let z = Node {
            directions: vec![(NfaItem::Content('z'), 1)],
        };
        let i = Node {
            directions: vec![(NfaItem::Content('i'), 2)],
        };
        let g = Node {
            directions: vec![(NfaItem::Content('g'), 3)],
        };
        let terminal = Node {
            directions: vec![(NfaItem::Terminal(Terminal::Zig), 0)],
        };
        let zig_nfa = Nfa::from_vec(vec![z, i, g, terminal]);

        // cxx nfa
        let c = Node {
            directions: vec![(NfaItem::Content('c'), 1)],
        };
        let x_1 = Node {
            directions: vec![(NfaItem::Content('x'), 2)],
        };
        let x_2 = Node {
            directions: vec![(NfaItem::Content('x'), 3)],
        };
        let terminal = Node {
            directions: vec![(NfaItem::Terminal(Terminal::CXX), 0)],
        };
        let cxx_nfa = Nfa::from_vec(vec![c, x_1, x_2, terminal]);

        rust_nfa.concat(0, ruby_nfa);
        rust_nfa.concat(0, zig_nfa);
        rust_nfa.concat(0, cxx_nfa);

        rust_nfa
    }

    #[test]
    fn test_rust_string() {
        let nfa = construct_nfa();
        let res = search::<char, Terminal>(&nfa, "rust");
        assert_eq!(vec![Terminal::Rust], res);
    }

    #[test]
    fn test_ruby_string() {
        let nfa = construct_nfa();
        let res = search::<char, Terminal>(&nfa, "ruby");
        assert_eq!(res, vec![Terminal::Ruby])
    }

    #[test]
    fn test_cxx_string() {
        let nfa = construct_nfa();
        let res = search(&nfa, "cxx");
        assert_eq!(res, vec![Terminal::CXX]);
    }

    #[test]
    fn test_zig_string() {
        let nfa = construct_nfa();
        let res = search(&nfa, "zig");
        assert_eq!(res, vec![Terminal::Zig]);
    }
}
