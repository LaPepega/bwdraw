 # bwdraw

 `bwdraw` is a Rust library designed for simple black and white 2D drawing in the terminal. It uses half-filled characters as pixels, allowing for a square-shaped representation without stretching the y-axis. The library provides a convenient way to draw with half-filled ASCII characters by representing the canvas as a grid of booleans and converting them into characters accordingly.

 ## Pixel Representation

 The library uses the concept of `DuoPixel`, where each pixel has upper and lower states. These states are converted into character representations using the [`Into<char>`] trait. The available characters for representation are:
 - `FULL_C`: Full-filled character ('█')
 - `UPPER_C`: Upper half-filled character ('▀')
 - `LOWER_C`: Lower half-filled character ('▄')
 - `EMPTY_C`: Empty character (' ')

 ## Examples

 ```rust
    // Draw a 10x10 square
    let height: usize = 10;
    let width: usize = 10;

    let mut square = Canvas::new(width, height);
    for i in 0..height {
        for j in 0..width {
            if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
                square.set(j, i, true)
            }
        }
    }
    println!("{}", square.to_string());
 ```

 ## Drawing Functions

 The library also provides a `clear` function, which clears the console screen using ANSI escape codes.