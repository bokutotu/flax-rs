use std::fmt::Debug;
use std::ops::{Index, IndexMut};

pub trait Content: PartialEq<char> + Clone + Copy + Debug + Sized {}
pub trait Terminal: Clone + Copy + Debug + Sized {}

pub trait State {
    type Terminal: Terminal;
    type Content: Content;
    fn is_terminal(&self) -> bool;
    fn is_content(&self) -> bool;
    fn terminal(&self) -> Self::Terminal;
    fn content(&self) -> Self::Content;
    fn from_content(content: Self::Content) -> Self;
    fn from_terminal(terminal: Self::Terminal) -> Self;
}

pub trait Node: IntoIterator + Default {
    type NodeState: State;

    fn add_transition(&mut self, transision: Self::NodeState, idx: usize);
    fn increment_all_index(&mut self, inc: usize);
    fn collect_terminal(&self) -> Vec<<Self::NodeState as State>::Terminal>;
    fn collect_content(&self) -> Vec<(<Self::NodeState as State>::Content, usize)>;

    fn collect_char_content_idx(&self, char_: char) -> Vec<usize> {
        self.collect_content()
            .iter()
            .filter(|(content, _)| *content == char_)
            .map(|(_, idx)| *idx)
            .collect()
    }

    fn add_content(&mut self, content: <Self::NodeState as State>::Content, idx: usize) {
        self.add_transition(Self::NodeState::from_content(content), idx);
    }

    fn add_terminal(&mut self, terminal: <Self::NodeState as State>::Terminal) {
        self.add_transition(Self::NodeState::from_terminal(terminal), 0);
    }

    fn from_content(content: <Self::NodeState as State>::Content, idx: usize) -> Self {
        let content_state = Self::NodeState::from_content(content);
        let mut defalut = Self::default();
        defalut.add_transition(content_state, idx);
        defalut
    }

    fn from_terminal(termianl: <Self::NodeState as State>::Terminal) -> Self {
        let mut defalut = Self::default();
        let terminal = Self::NodeState::from_terminal(termianl);
        defalut.add_transition(terminal, 0);
        defalut
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Automaton<N> {
    nodes: Vec<N>,
}

impl<N: Node> Index<usize> for Automaton<N> {
    type Output = N;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.nodes[idx]
    }
}

impl<N: Node> IndexMut<usize> for Automaton<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl<N: Node> Automaton<N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, node: N) {
        self.nodes.push(node);
    }

    pub fn from_content(content: <N::NodeState as State>::Content) -> Self {
        let node = N::from_content(content, 1);
        let mut automata = Self::new();
        automata.push(node);
        automata.push(N::default());
        automata
    }

    pub fn from_terminal(terminal: <N::NodeState as State>::Terminal) -> Self {
        let node = N::from_terminal(terminal);
        let mut automata = Self::new();
        automata.push(node);
        automata
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn increment_all_index(self, inc: usize) -> Self {
        let mut res = self;
        res.nodes
            .iter_mut()
            .for_each(|x| x.increment_all_index(inc));
        res
    }

    pub fn add_state_idx_node(
        &mut self,
        idx: usize,
        transition: N::NodeState,
        direction_idx: usize,
    ) {
        self.nodes[idx].add_transition(transition, direction_idx);
    }

    pub fn add_terminal_idx_node(
        &mut self,
        idx: usize,
        terminal: <N::NodeState as State>::Terminal,
    ) {
        if self.len() == 0 {
            panic!("automaton length 0 so, You cannot call add_terminal_idx_node");
        }
        self.nodes[idx].add_terminal(terminal);
    }

    pub fn add_content_idx_node(
        &mut self,
        idx: usize,
        content: <N::NodeState as State>::Content,
        direction_idx: usize,
    ) {
        self.nodes[idx].add_content(content, direction_idx);
    }

    pub fn append_vec(&mut self, other: Self) {
        let mut other = other.nodes;
        self.nodes.append(&mut other);
    }

    pub fn set_terminal_to_idx(&mut self, idx: usize, terminal: <N::NodeState as State>::Terminal) {
        self.index_mut(idx).add_terminal(terminal);
    }

    pub fn set_termial_to_last_node(&mut self, terminal: <N::NodeState as State>::Terminal) {
        let current_len = self.len() - 1;
        self.set_terminal_to_idx(current_len, terminal);
    }
}

pub trait NextNode {
    type InputState: State;
    fn next_node(&self, idx: usize, char_: char) -> Vec<usize>;
}

pub trait RegexRun<N: Node>: NextNode + Index<usize, Output = N> {
    fn run_inner(
        &self,
        char_vec: &[char],
        idx: usize,
    ) -> Vec<<<N as Node>::NodeState as State>::Terminal> {
        println!("{:?}", idx);
        println!("{:?}", char_vec);
        let mut terminals = self[idx].collect_terminal();
        if !char_vec.is_empty() {
            println!("{:?}", self.next_node(idx, char_vec[0]));
            self.next_node(idx, char_vec[0])
                .iter()
                .for_each(|next_idx| {
                    if *next_idx != 0 {
                        let mut next_char_res = self.run_inner(&char_vec[1..], *next_idx);
                        terminals.append(&mut next_char_res);
                    }
                });
        }
        terminals
    }

    fn run(&self, search_string: &str) -> Vec<<<N as Node>::NodeState as State>::Terminal> {
        let char_vec = search_string.chars().collect::<Vec<_>>();
        self.run_inner(&char_vec, 0)
    }
}
