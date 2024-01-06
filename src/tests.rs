use super::*;

#[test]
fn from_even_vec_of_bools_to_canvas() {
    let bools = vec![
        vec![true, true, false, false],
        vec![true, false, true, false],
    ];
    let expected = Canvas(vec![Row(vec![
        DuoPixel {
            upper: true,
            lower: true,
        },
        DuoPixel {
            upper: true,
            lower: false,
        },
        DuoPixel {
            upper: false,
            lower: true,
        },
        DuoPixel {
            upper: false,
            lower: false,
        },
    ])]);
    assert_eq!(Canvas::from(bools), expected);
}

#[test]
fn from_odd_vec_of_bools_to_canvas() {
    let bools = vec![
        vec![true, false, true, false],
        vec![false, true, false, true],
        vec![true, false, true, false],
    ];
    let expected = Canvas(vec![
        Row(vec![
            DuoPixel {
                upper: true,
                lower: false,
            },
            DuoPixel {
                upper: false,
                lower: true,
            },
            DuoPixel {
                upper: true,
                lower: false,
            },
            DuoPixel {
                upper: false,
                lower: true,
            },
        ]),
        Row(vec![
            DuoPixel {
                upper: true,
                lower: false,
            },
            DuoPixel {
                upper: false,
                lower: false,
            },
            DuoPixel {
                upper: true,
                lower: false,
            },
            DuoPixel {
                upper: false,
                lower: false,
            },
        ]),
    ]);
    assert_eq!(Canvas::from(bools), expected);
}

#[test]
fn from_empty_input_to_canvas() {
    let input: Vec<Vec<bool>> = Vec::new();

    let expected_output = "";

    let picture: Canvas = input.into();
    let output_string: String = picture.into();

    assert_eq!(output_string, expected_output);
}
