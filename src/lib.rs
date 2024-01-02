const FULL_C: char = '█';
const LOWER_C: char = '▄';
const UPPER_C: char = '▀';
const EMPTY_C: char = ' ';

#[derive(Debug, Clone)]
struct Pixel {
    upper: bool,
    lower: bool,
}

impl From<(bool, bool)> for Pixel {
    fn from(value: (bool, bool)) -> Self {
        Pixel {
            upper: value.0,
            lower: value.1,
        }
    }
}

impl Into<char> for Pixel {
    fn into(self) -> char {
        match (self.lower, self.upper) {
            (true, true) => FULL_C,
            (false, true) => UPPER_C,
            (true, false) => LOWER_C,
            (false, false) => EMPTY_C,
        }
    }
}

#[derive(Debug, Clone)]
struct Row(Vec<Pixel>);

impl From<(Vec<bool>, Vec<bool>)> for Row {
    fn from(value: (Vec<bool>, Vec<bool>)) -> Self {
        let pixels: Vec<Pixel> = value
            .0
            .iter()
            .zip(value.1.iter())
            .map(|(&u, &l)| Pixel { upper: u, lower: l })
            .collect();
        Row(pixels)
    }
}

impl Into<String> for Row {
    fn into(self) -> String {
        self.0
            .iter()
            .cloned()
            .map(|p| {
                let c: char = p.into();
                c
            })
            .collect()
    }
}

struct Picture(Vec<Row>);

impl From<Vec<Vec<bool>>> for Picture {
    fn from(value: Vec<Vec<bool>>) -> Self {
        // add a vec of falses if number of subpixels is false
        let longed = if value.len() % 2 == 0 {
            value
        } else {
            let inner_len = if let Some(inner) = value.get(0) {
                inner.len()
            } else {
                0
            };
            let falses_vec: Vec<bool> = vec![false; inner_len];
            let mut new_value = value.clone();
            new_value.push(falses_vec);
            new_value
        };
        let paired: Vec<(Vec<bool>, Vec<bool>)> = longed
            .chunks(2)
            .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
            .collect();
        let rows = paired.iter().map(|p| Row::from(p.clone())).collect();
        Picture(rows)
    }
}

impl Into<String> for Picture {
    fn into(self) -> String {
        self.0
            .iter()
            .cloned()
            .map(|r| {
                let s: String = r.into();
                s + "\n"
            })
            .collect()
    }
}

pub fn clear() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_from_vec_of_bools_to_picture() {
        todo!()
    }

    #[test]
    fn test_conversion_from_odd_number_of_rows_to_picture() {
        todo!()
    }

    #[test]
    fn test_conversion_from_empty_input_to_picture() {
        let input: Vec<Vec<bool>> = Vec::new();

        let expected_output = "";

        let picture: Picture = input.into();
        let output_string: String = picture.into();

        assert_eq!(output_string, expected_output);
    }
}
