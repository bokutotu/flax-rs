// #[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
// pub enum DfaItem<C, T> {
//     Content(C),
//     Terminal(T)
// }
//
// impl<C, T> DfaItem<C, T> {
//     fn is_terminal(&self) -> bool {
//         matches!(self, DfaItem::Terminal(_))
//     }
//
//     fn from_content(content: C) -> Self {
//         DfaItem::Content(content)
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct DfaNode<C, T> {
//     directions: Vec<(DfaItem<C, T>, usize)>,
// }
//
// impl<C, T: Clone> DfaNode<C, T> {
//     fn new() -> Self {
//         Self {
//             directions: Vec::new(),
//         }
//     }
//     fn add_direction(&mut self, dir_char: DfaItem<C, T>, idx: usize) {
//         self.directions.push((dir_char, idx));
//     }
//
//     pub fn add_content(&mut self, content: C, idx: usize) {
//         let item = DfaItem::Content(content);
//         self.add_direction(item, idx);
//     }
//
//     pub fn add_terminal(&mut self, terminal: T) {
//         let item = DfaItem::Terminal(terminal);
//         self.add_direction(item, 0);
//     }
//
//     fn increment_all_index(&mut self, inc: usize) {
//         self.directions.iter_mut().for_each(|(_, idx)| *idx += inc);
//     }
//
//     fn from_content(content: C) -> Self {
//         let item = DfaItem::from_content(content);
//         DfaNode {
//             directions: vec![(item, 1)],
//         }
//     }
// }
//
// #[derive(Clone, PartialEq, Eq, Debug)]
// pub struct Dfa<C, T> {
//     nfa: Vec<DfaNode<C, T>>,
//     len: usize,
// }
