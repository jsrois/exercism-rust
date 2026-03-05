pub fn answer(command: &str) -> Option<i32> {
    let words: Vec<&str> = command.strip_suffix('?').unwrap().split(' ').collect();
    match words.as_slice() {
        ["What", "is", lhs, rest @ ..] => parse_ops(str::parse::<i32>(lhs).ok()?, rest),
        _ => None,
    }
}

fn parse_ops(lhs: i32, rest: &[&str]) -> Option<i32> {
    match rest {
        ["plus", n, rest @ ..] => parse_ops(lhs + str::parse::<i32>(n).ok()?, rest),
        ["minus", n, rest @ ..] => parse_ops(lhs - str::parse::<i32>(n).ok()?, rest),
        ["multiplied", "by", n, rest @ ..] => parse_ops(lhs * str::parse::<i32>(n).ok()?, rest),
        ["divided", "by", n, rest @ ..] => parse_ops(lhs / str::parse::<i32>(n).ok()?, rest),
        [] => Some(lhs),
        _ => None,
    }
}
