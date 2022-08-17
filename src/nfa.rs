use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub(crate) enum NfaItem<T> {
    Epsilon,
    Content(char),
    Terminal(T)
}

impl<T> NfaItem<T> {
    fn is_epsilon(&self) -> bool {
        match self {
            NfaItem::Epsilon => true,
            _ => false
        }
    }

    fn is_content(&self) -> bool {
        match self {
            NfaItem::Content(_) => true,
            _ => false
        }
    }

    fn is_terminal(&self) -> bool {
        match self {
            NfaItem::Terminal(_) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    directions: Vec<(NfaItem<T>, usize)>
}

impl<T> Node<T> {
    fn add_direction(&mut self, dir_char: NfaItem<T>, idx: usize) {
        self.directions.push((dir_char, idx));
    }

    fn increment_all_index(&mut self, inc: usize) {
        self.directions
            .iter_mut()
            .for_each(|(_, idx)| *idx += inc);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Nfa<T> {
    nfa: Vec<Node<T>>,
    len: usize,
}

impl<T: Clone> Nfa<T> {
    pub fn new() -> Self {
        Nfa { nfa: Vec::new(), len: 0 }
    }

    pub fn push(&mut self, node: Node<T>) {
        self.nfa.push(node);
    }

    fn increment_all_node_idx(&mut self, inc: usize) {
        self.nfa.iter_mut().for_each(|node| node.increment_all_index(inc));
    }

    fn clone_vec(&self) -> Vec<Node<T>> {
        self.nfa.clone()
    }

    fn update_len(&mut self) {
        self.len = self.nfa.len();
    }

    fn len(&self) -> usize {
        self.nfa.len()
    }

    /// Connect another Node's NFA to any NFA node.
    /// 1. add all the idx of the nfa to be connected by the original length
    /// 2. update the length of the original nfa
    /// 3. connect the first index of the nfa to the node to be connected with 
    ///    an arbitrary NfaItem
    /// 4. update length of NFA
    pub fn concat(&mut self, source_idx: usize, cat_nfa: &mut Nfa<T>) {
        cat_nfa.increment_all_node_idx(self.len());
        let mut cat_nfa_vec = cat_nfa.clone_vec();
        self.nfa[source_idx].add_direction(NfaItem::Epsilon, self.len);
        self.nfa.append(&mut cat_nfa_vec);
        self.update_len();
    }
}

fn search_inner<T: Copy>(
    nfa: &Nfa<T>, 
    search_char_vec: &[char], 
    idx: usize
    )
    -> Vec<T> 
{
    let mut res = Vec::new();
    if search_char_vec.len() == 0 {
        return nfa[idx]
            .directions
            .iter()
            .filter(|(item, _)| {
                item.is_terminal()
            })
            .map(|(item, _)| {
                if let NfaItem::Terminal(x) = *item { return x }
                unreachable!()
            })
            .collect::<Vec<T>>();
        
    }

    let tmp_char = search_char_vec[0];
    let ref next_search_char_vec = search_char_vec[1..];

    nfa[idx].directions.iter().for_each(|(item, idx)| {
        match item {
            NfaItem::Terminal(x) => res.push(*x),
            NfaItem::Epsilon => {
                let mut tmp = search_inner(nfa, search_char_vec, *idx);
                res.append(&mut tmp);
            },
            NfaItem::Content(x) => {
                if *x == tmp_char {
                    let mut tmp = search_inner(nfa, next_search_char_vec, *idx);
                    res.append(&mut tmp);
                }
            }
        }
    });

    res
}

pub fn search<T: Copy + Debug + PartialEq>(nfa: &Nfa<T>, search_string: &str) -> Vec<T> {
    let search_char_vec: Vec<char> = search_string.chars().collect::<Vec<_>>();
    search_inner(nfa, &search_char_vec, 0)
}

impl<T> Index<usize> for Nfa<T> {
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nfa[index]
    }
}

impl<T> IndexMut<usize> for Nfa<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nfa[index]
    }
}

impl<'a, T> Index<usize> for &'a Nfa<T>{
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nfa[index]
    }
}

impl<'a, T> Index<usize> for &'a mut Nfa<T> {
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nfa[index]
    }
}

impl<'a, T> IndexMut<usize> for &'a mut Nfa<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nfa[index]
    }
}

#[cfg(test)]
mod nfa_trasition_test {
    use crate::nfa::{NfaItem, Node, Nfa, search};

    impl<T> Nfa<T> {
        fn from_vec(nfa: Vec<Node<T>>) -> Self {
            let len = nfa.len();
            Self { nfa, len }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Terminal {
        Rust,
        Zig,
        Ruby,
        CXX
    }

    fn construct_nfa() -> Nfa<Terminal> {
        // rust nfa
        let first = Node { directions: vec![(NfaItem::Epsilon, 1)] };
        let r = Node { directions: vec![(NfaItem::Content('r'), 2)] };
        let u = Node { directions: vec![(NfaItem::Content('u'), 3)] };
        let s = Node { directions: vec![(NfaItem::Content('s'), 4)] };
        let t = Node { directions: vec![(NfaItem::Content('t'), 5)] };
        let terminal = Node { directions: vec![(NfaItem::Terminal(Terminal::Rust), 0)] };
        let mut rust_nfa = Nfa::from_vec(vec![first, r, u, s ,t, terminal ]);

        // ruby nfa
        let r = Node { directions: vec![(NfaItem::Content('r'), 1)] };
        let u = Node { directions: vec![(NfaItem::Content('u'), 2)] };
        let b = Node { directions: vec![(NfaItem::Content('b'), 3)] };
        let y = Node { directions: vec![(NfaItem::Content('y'), 4)] };
        let terminal = Node { directions: vec![(NfaItem::Terminal(Terminal::Ruby), 0)] };
        let mut ruby_nfa = Nfa::from_vec(vec![r, u, b, y, terminal]);

        // zig nfa
        let z = Node { directions: vec![(NfaItem::Content('z'), 1)] };
        let i = Node { directions: vec![(NfaItem::Content('i'), 2)] };
        let g = Node { directions: vec![(NfaItem::Content('g'), 3)] };
        let terminal = Node { directions: vec![(NfaItem::Terminal(Terminal::Zig), 0)] };
        let mut zig_nfa = Nfa::from_vec(vec![z, i, g, terminal]);

        // cxx nfa
        let c = Node { directions: vec![(NfaItem::Content('c'), 1)] };
        let x_1 = Node { directions: vec![(NfaItem::Content('x'), 2)] };
        let x_2 = Node { directions: vec![(NfaItem::Content('x'), 3)] };
        let terminal = Node { directions: vec![(NfaItem::Terminal(Terminal::CXX), 0)] };
        let mut cxx_nfa = Nfa::from_vec(vec![c, x_1, x_2, terminal]);

        rust_nfa.concat(0, &mut ruby_nfa);
        rust_nfa.concat(0, &mut zig_nfa);
        rust_nfa.concat(0, &mut cxx_nfa);

        rust_nfa
    }

    

    #[test]
    fn test_rust_string() {
        let nfa = construct_nfa();
        let res = search::<Terminal>(&nfa, "rust");
        assert_eq!(vec![Terminal::Rust], res);
    }
}
