pub fn recite(start_bottles: u32, take_down: u32) -> String {
    fn num(n: u32) -> String {
        match n {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            _ => "ten",
        }
        .to_string()
    }
    fn bottle_string(n: u32) -> String {
        match n {
            0 => "no green bottles".to_string(),
            1 => "one green bottle".to_string(),
            _ => format!("{} green bottles", num(n)),
        }
    }
    fn capitalize(s: String) -> String {
        s.chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_string()
                }
            })
            .collect()
    }

    (1..=start_bottles)
        .rev()
        .take(take_down as usize)
        .map(|num_bottles| {
            format!(
                r#"{} hanging on the wall,
{} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {} hanging on the wall.
"#,
                capitalize(bottle_string(num_bottles)),
                capitalize(bottle_string(num_bottles)),
                bottle_string(num_bottles - 1)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
