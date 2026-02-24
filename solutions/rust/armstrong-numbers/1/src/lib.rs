pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_string = num.to_string();
    let s: u32 = num_as_string
        .chars()
        .rev()
        .map(|x| unsafe {
            x.to_digit(10)
                .unwrap_unchecked()
                .pow(num_as_string.len() as u32)
        })
        .sum();

    s == num
}
