//! 作りたいルールの一覧
//! 1. or a|b -> aとb両方とも受理する
//! 2. 括りだし {adfd} -> adfdを受理する
//! 3. 回数指定繰り返し -> {2, 3}とか*など repに対応するもの
//! 文法の優先順位を考える
//! 一番低い文法をcharsとする
//! expr = (word { ( "|" world ) | rep }?)*
//! word = ( ors | Alphabet ) * | "(" expr ")"
//! ors = [Alphabet * ]
//! Alphabet = a-z | A-Z | 0 - 9 | 記号
//! ユニットテストはしたいけど、結合テストメインで行う
//! TODO ユニットテスト
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

use crate::regex_tokenizer::{Item, RegexTokenIter};
use crate::nfa::{NfaEdge, NfaNode};

macro_rules! not_alphabet_set {
    () => {
        [
        Item::OneOrMore, 
        Item::Any, 
        Item::SomeTime, 
        Item::Or, 
        Item::ZeroOrOne, 
        Item::BracketL, 
        Item::BracketR, 
        Item::CurryL, 
        Item::CurryR, 
        Item::SquareL, 
        Item::SquareR
        ]
    }
}

pub fn alphabet<T: Clone + Debug>(iter: &mut RegexTokenIter) -> Option<(Rc<RefCell<NfaNode<T>>>, Rc<RefCell<NfaNode<T>>>)> {
    let next_token = iter.next()?;
    if not_alphabet_set!().contains(&next_token) {
        None
    } else {
        let edge = NfaEdge::new_alphabet(next_token);
        let mut node = NfaNode::new_non_terminal();
        let child = Default::default();
        node.add_child(edge, Rc::clone(&child));
        Some((Rc::new(RefCell::new(node)), child))
    }
}
