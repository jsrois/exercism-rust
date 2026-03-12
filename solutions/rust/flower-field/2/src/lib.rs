pub fn annotate(garden: &[&str]) -> Vec<String> {
    (0..garden.len())
        .map(|row| {
            (0..garden[row].len())
                .map(|column| calculate_count(garden, row as i32, column as i32))
                .collect()
        })
        .collect()
}

fn calculate_count(garden: &[&str], r: i32, c: i32) -> char {
    match garden[r as usize].as_bytes()[c as usize] {
        b'*' => '*',
        _ => match count_adjacent_flowers(garden, r, c) {
            0 => ' ',
            n => (n as u8 + b'0') as char,
        },
    }
}

fn count_adjacent_flowers(garden: &[&str], r: i32, c: i32) -> usize {
    let max_row = (garden.len() - 1) as i32;
    let max_col = (garden[r as usize].len() - 1) as i32;
    ((r - 1).max(0)..=(r + 1).min(max_row))
        .flat_map(|row| ((c - 1).max(0)..=(c + 1).min(max_col)).map(move |col| (row, col)))
        .filter(|&(r, c)| garden[r as usize].as_bytes()[c as usize] == b'*')
        .count()
}
