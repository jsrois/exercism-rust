use regex::Regex;

pub fn answer(command: &str) -> Option<i32> {
    let sum_expr = Regex::new("What is (-?[0-9]+) plus (-?[0-9]+)\\?").unwrap();
    let sub_expr = Regex::new("What is (-?[0-9]+) minus (-?[0-9]+)\\?").unwrap();
    let mult_expr = Regex::new("What is (-?[0-9]+) multiplied by (-?[0-9]+)\\?").unwrap();
    let div_expr = Regex::new("What is (-?[0-9]+) divided by (-?[0-9]+)\\?").unwrap();
    let num_expr = Regex::new("What is (-?[0-9]+)\\?").unwrap();
    if num_expr.is_match(command) {
        num_expr
            .captures(command)
            .map(|c| c[1].parse::<i32>().unwrap())
    } else if sum_expr.is_match(command) {
        sum_expr
            .captures(command)
            .map(|c| c[1].parse::<i32>().unwrap() + c[2].parse::<i32>().unwrap())
    } else if sub_expr.is_match(command) {
        sub_expr
            .captures(command)
            .map(|c| c[1].parse::<i32>().unwrap() - c[2].parse::<i32>().unwrap())
    } else if mult_expr.is_match(command) {
        mult_expr
            .captures(command)
            .map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
    } else {
        div_expr
            .captures(command)
            .map(|c| c[1].parse::<i32>().unwrap() / c[2].parse::<i32>().unwrap())
    }
}
