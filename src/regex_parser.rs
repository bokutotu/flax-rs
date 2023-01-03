//! 作りたいルールの一覧
//! 1. or a|b -> aとb両方とも受理する
//! 2. 括りだし {adfd} -> adfdを受理する
//! 3. 回数指定繰り返し -> {2, 3}とか*など repに対応するもの
//! 文法の優先順位を考える
//! 一番低い文法をcharsとする
//! expr = chars ( ("|" chars) | (rep) ) ?
//! chars = a-z,A-Z, 0-9, backslashs, "{" expr "}"
