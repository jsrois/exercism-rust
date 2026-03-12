use std::iter::zip;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contained(longer_list: &[i32], shorter_list: &[i32]) -> bool {
    shorter_list.is_empty()
        || longer_list
            .windows(shorter_list.len())
            .any(|s| s == shorter_list)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list, second_list) {
        (f, s) if f == s => Comparison::Equal,
        (f, s) if contained(f, s) => Comparison::Superlist,
        (f, s) if contained(s, f) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}
