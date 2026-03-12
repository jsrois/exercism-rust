/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.replace(" ", "").len() <= 1 {
        return false;
    }
    if code.chars().any(|c| !c.is_whitespace() && !c.is_numeric()) {
        return false;
    }
    let s: u32 = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .map(|d| d as u32 - '0' as u32)
        .enumerate()
        .map(|(i, val)| if i % 2 == 0 { val } else { 2 * (val) })
        .map(|d| if d > 9 { d - 9 } else { d })
        .sum();

    s.is_multiple_of(10)
}
