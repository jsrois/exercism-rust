pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.0.replace(" ", "").len() <= 1 {
            return false;
        }
        let s: u32 = self
            .0
            .chars()
            .filter(|c| !c.is_whitespace())
            .rev()
            .map(|d| d as u32 - '0' as u32)
            .enumerate()
            .map(|(i, val)| if i % 2 == 0 { val } else { 2 * val })
            .map(|n| if n > 9 { n - 9 } else { n })
            .sum();
        s.is_multiple_of(10)
    }
}

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
