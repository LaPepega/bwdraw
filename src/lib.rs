const FULL: char = '█';
const LOWER: char = '▄';
const UPPER: char = '▀';
const EMPTY: char = ' ';

pub fn clear() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[cfg(test)]
mod tests {
    use super::*;
}
