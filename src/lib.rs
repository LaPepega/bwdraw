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
//! use bwdraw::Canvas;
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

use std::ops::Deref;

#[cfg(test)]
mod tests;

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

impl Into<(bool, bool)> for DuoPixel {
    fn into(self) -> (bool, bool) {
        (self.upper, self.lower)
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

impl PartialEq for DuoPixel {
    fn eq(&self, other: &Self) -> bool {
        self.upper == other.upper && self.lower == other.lower
    }
}

/// Represents a row of pixels in the drawing canvas.
///
/// Each row is composed of a vector of `Pixel` instances and
/// can be converted into a string using the `Into<String>` trait.
#[derive(Debug, Clone)]
pub struct Row(Vec<DuoPixel>);

impl Deref for Row {
    type Target = Vec<DuoPixel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
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

    /// Sets a [`DuoPixel`] on [`Canvas`] to specified one and return [`DuoPixel`] which was previously there.
    /// Returns [`None`] if `(x,y)` is out of bounds
    pub fn mut_set_duopixel(&mut self, x: usize, y: usize, pixel: DuoPixel) -> Option<DuoPixel> {
        let original = self.0.get_mut(y)?.0.get_mut(x)?;
        let orig = original.clone();
        *original = pixel;
        Some(orig)
    }

    /// Get [`DuoPixel`] at `(x,y)`
    /// Returns [`None`] if `(x,y)` is out of bounds
    pub fn get_duopixel(&self, x: usize, y: usize) -> Option<DuoPixel> {
        let pix = self.0.get(y)?.0.get(x)?;
        Some(pix.clone())
    }

    /// Inverts state of pixel at `(x,y)` on existing Canvas and returns resulting Canvas
    /// Returns [`None`] if `(x,y)` is out of bounds
    pub fn mut_invert_pixel(&mut self, x: usize, y: usize) -> Option<Canvas> {
        let mut subpixeled: Vec<Vec<bool>> = self.clone().into();
        let orig = subpixeled.get_mut(y)?.get_mut(x)?;
        *orig = !orig.clone();

        let new_pic = Canvas::from(subpixeled);

        *self = new_pic.clone();
        Some(new_pic)
    }

    /// Returns new Canvas with inverted pixel at `(x,y)`
    /// Returns [`None`] if `(x,y)` is out of bounds
    pub fn invert_pixel(&self, x: usize, y: usize) -> Option<Canvas> {
        let mut subpixeled: Vec<Vec<bool>> = self.clone().into();
        let orig = subpixeled.get_mut(y)?.get_mut(x)?;
        *orig = !orig.clone();

        let new_pic = Canvas::from(subpixeled);

        Some(new_pic)
    }

    /// Sets a state of square pixel on existing [`Canvas`] and returns the resulting [`Canvas`].
    /// Returns [`None`] if `(x,y)` is out of bounds
    pub fn mut_set(&mut self, x: usize, y: usize, state: bool) -> Option<Self> {
        let mut subpixeled: Vec<Vec<bool>> = self.clone().into();
        *subpixeled.get_mut(y)?.get_mut(x)? = state;

        let new_pic = Canvas::from(subpixeled);

        *self = new_pic.clone();
        Some(new_pic)
    }

    /// Returns a new canvas with set state of square pixel at `(x,y)`
    /// Returns [`None`] if `(x,y)` is out of bounds
    pub fn set(&self, x: usize, y: usize, state: bool) -> Option<Self> {
        let mut subpixeled: Vec<Vec<bool>> = self.clone().into();
        *subpixeled.get_mut(y)?.get_mut(x)? = state;
        let new_pic = Canvas::from(subpixeled);
        Some(new_pic)
    }

    /// Gets state of square pixel at `(x,y)`.
    /// Returns [`None`] if `(x,y)` is out of bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        let subpixeled: Vec<Vec<bool>> = self.clone().into();
        Some(subpixeled.get(y)?.get(x)?.clone())
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

    pub fn invert(&mut self) {
        let subpixeled: Vec<Vec<bool>> = self.clone().into();
        let inverted: Vec<Vec<bool>> = subpixeled
            .iter()
            .map(|r| r.iter().map(|p| !p).collect())
            .collect();
        *self = inverted.into();
    }

    /// Returns inverted [`Canvas`]
    pub fn inverted(&self) -> Self {
        let subpixeled: Vec<Vec<bool>> = self.clone().into();
        let inverted: Vec<Vec<bool>> = subpixeled
            .iter()
            .map(|r| r.iter().map(|p| !p).collect())
            .collect();
        inverted.into()
    }
}

impl Deref for Canvas {
    type Target = Vec<Row>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for Canvas {
    fn to_string(&self) -> String {
        let s: String = self.clone().into();
        s
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

impl Into<Vec<Row>> for Canvas {
    fn into(self) -> Vec<Row> {
        self.0
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

impl PartialEq for Canvas {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Clears the console screen.
///
/// This function sends ANSI escape codes to clear the console screen.
pub fn clear() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
