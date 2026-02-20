use std::iter::zip;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contained(longer_list: &[i32], shorter_list: &[i32]) -> bool {
    let sll = shorter_list.len();
    for (i, _) in longer_list.iter().enumerate() {
        if sll <= longer_list[i..].len() {
            let contained =
                zip(shorter_list.iter(), longer_list[i..i + sll].iter()).all(|t| t.0 == t.1);
            if contained {
                return true;
            }
        }
    }
    false
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list, second_list) {
        (f, s) if f == s => Comparison::Equal,
        (f, s) if f.len() < s.len() && contained(s, f) => Comparison::Sublist,
        (f, s) if (f.len() >= s.len()) && contained(f, s) => Comparison::Superlist,
        (_, _) => Comparison::Unequal,
    }
}
