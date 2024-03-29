#[derive(Debug, PartialEq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    use ResistorColor::*;
    match _color {
        Black => 0,
        Brown => 1,
        Red => 2,
        Orange => 3,
        Yellow => 4,
        Green => 5,
        Blue => 6,
        Violet => 7,
        Grey => 8,
        White => 9,
    }
}

pub fn value_to_color_string(value: usize) -> String {
    String::from(match value {
        0 => "Black",
        1 => "Brown",
        2 => "Red",
        3 => "Orange",
        4 => "Yellow",
        5 => "Green",
        6 => "Blue",
        7 => "Violet",
        8 => "Grey",
        9 => "White",
        _ => "value out of range",
    })
}

pub fn colors() -> Vec<ResistorColor> {
    use ResistorColor::*;
    vec![
        Black,
        Brown,
        Red,
        Orange,
        Yellow,
        Green,
        Blue,
        Violet,
        Grey,
        White,
    ]
}
