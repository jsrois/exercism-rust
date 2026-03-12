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
    if first_list == second_list {
        return Comparison::Equal;
    }
    if first_list.len() < second_list.len() && contained(second_list, first_list) {
        return Comparison::Sublist;
    }
    if first_list.len() >= second_list.len() && contained(first_list, second_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}
