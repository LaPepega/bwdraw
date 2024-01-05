//! # bwdraw
//!
//! `bwdraw` is a Rust library designed for simple black and white 2D drawing in the terminal. It uses half-filled characters as pixels, allowing for a square-shaped representation without stretching the y-axis. The library provides a convenient way to draw with half-filled ASCII characters by representing the canvas as a grid of booleans and converting them into characters accordingly.
//!
//! ## Pixel Representation
//!
//! The library uses the concept of `DuoPixel`, where each pixel has upper and lower states. These states are converted into character representations using the [`Into<char>`] trait. The available characters for representation are:
//! - `FULL_C`: Full-filled character ('█')
//! - `UPPER_C`: Upper half-filled character ('▀')
//! - `LOWER_C`: Lower half-filled character ('▄')
//! - `EMPTY_C`: Empty character (' ')
//!
//! ## Examples
//!
//! ```rust
//!    // Draw a 10x10 square
//!    let height: usize = 10;
//!    let width: usize = 10;
//!
//!    let mut square = Canvas::new(width, height);
//!    for i in 0..height {
//!        for j in 0..width {
//!            if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
//!                square.set(j, i, true)
//!            }
//!        }
//!    }
//!    println!("{}", square.to_string());
//! ```
//!
//! ## Drawing Functions
//!
//! The library also provides a `clear` function, which clears the console screen using ANSI escape codes.

pub const FULL_C: char = '\u{2588}';
pub const LOWER_C: char = '\u{2584}';
pub const UPPER_C: char = '\u{2580}';
pub const EMPTY_C: char = ' ';

/// Represents a single pixel in the drawing canvas.
///
/// Each pixel can have an upper and lower state, to be converted into a character
/// representation based on its state using the [`Into<char>`] trait.
#[derive(Debug, Clone)]
pub struct DuoPixel {
    upper: bool,
    lower: bool,
}

impl From<(bool, bool)> for DuoPixel {
    fn from(value: (bool, bool)) -> Self {
        DuoPixel {
            upper: value.0,
            lower: value.1,
        }
    }
}

impl Into<char> for DuoPixel {
    fn into(self) -> char {
        match (self.lower, self.upper) {
            (true, true) => FULL_C,
            (false, true) => UPPER_C,
            (true, false) => LOWER_C,
            (false, false) => EMPTY_C,
        }
    }
}

/// Represents a row of pixels in the drawing canvas.
///
/// Each row is composed of a vector of `Pixel` instances and
/// can be converted into a string using the `Into<String>` trait.
#[derive(Debug, Clone)]
pub struct Row(Vec<DuoPixel>);

impl From<(Vec<bool>, Vec<bool>)> for Row {
    fn from(value: (Vec<bool>, Vec<bool>)) -> Self {
        let pixels: Vec<DuoPixel> = value
            .0
            .iter()
            .zip(value.1.iter())
            .map(|(&u, &l)| DuoPixel { upper: u, lower: l })
            .collect();
        Row(pixels)
    }
}

impl Into<(Vec<bool>, Vec<bool>)> for Row {
    fn into(self) -> (Vec<bool>, Vec<bool>) {
        self.0
            .into_iter()
            .map(|pixel| (pixel.upper, pixel.lower))
            .unzip()
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

/// Represents the drawing canvas, composed of rows of pixels.
///
/// The canvas can be initialized with a specified width and height, and it provides methods
/// for modifying and converting its content.
#[derive(Debug, Clone)]
pub struct Canvas(Vec<Row>);

impl Canvas {
    /// Creates new empty [`Canvas`] with set `width` and `height`
    pub fn new(width: usize, height: usize) -> Self {
        Canvas::from(vec![vec![false; width]; height])
    }

    /// Returns [`String`] representation of [`Canvas`]
    pub fn to_string(&self) -> String {
        let s: String = self.clone().into();
        s
    }

    /// Sets a [`DuoPixel`] on [`Canvas`] to specified one
    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: DuoPixel) {
        self.0[y].0[x] = pixel;
    }

    /// Sets a state of square pixel on canvas
    pub fn set(&mut self, x: usize, y: usize, state: bool) {
        let mut subpixeled: Vec<Vec<bool>> = self.clone().into();
        subpixeled[y][x] = state;
        let new_pic = Canvas::from(subpixeled);
        *self = new_pic;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let subpixeled: Vec<Vec<bool>> = self.clone().into();
        subpixeled[y][x]
    }

    /// Parse canvas from string specifying chars representing active and inactive pixels.
    /// Any unspecified chars will be interpreted as active
    pub fn parse(str_pic: &str, active: char, inactive: char) -> Self {
        str_pic
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        if c == active {
                            true
                        } else if c == inactive {
                            false
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>()
            .into()
    }
}

impl Into<Vec<Vec<bool>>> for Canvas {
    fn into(self) -> Vec<Vec<bool>> {
        self.0
            .into_iter()
            .flat_map(|row| {
                let t: (Vec<bool>, Vec<bool>) = row.into();
                vec![t.0, t.1]
            })
            .collect()
    }
}

impl From<Vec<Vec<bool>>> for Canvas {
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
        Canvas(rows)
    }
}

impl Into<String> for Canvas {
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

/// Clears the console screen.
///
/// This function sends ANSI escape codes to clear the console screen.
pub fn clear() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_even_vec_of_bools_to_canvas() {
        todo!()
    }

    #[test]
    fn from_odd_vec_of_bools_to_canvas() {
        todo!()
    }

    #[test]
    fn from_empty_input_to_canvas() {
        let input: Vec<Vec<bool>> = Vec::new();

        let expected_output = "";

        let picture: Canvas = input.into();
        let output_string: String = picture.into();

        assert_eq!(output_string, expected_output);
    }
}
