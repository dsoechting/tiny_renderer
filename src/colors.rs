use crate::tga::RGBA;

const WHITE: RGBA = RGBA {
    r: 255,
    b: 255,
    g: 255,
    a: 255,
};

const BLUE: RGBA = RGBA {
    r: 0,
    b: 255,
    g: 0,
    a: 255,
};

const RED: RGBA = RGBA {
    r: 255,
    b: 0,
    g: 0,
    a: 255,
};

const GREEN: RGBA = RGBA {
    r: 0,
    b: 0,
    g: 255,
    a: 255,
};

const YELLOW: RGBA = RGBA {
    r: 255,
    b: 0,
    g: 200,
    a: 255,
};

#[derive(Debug, Copy, Clone)]
pub enum Color {
    White,
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    pub fn rgba_value(&self) -> RGBA {
        match self {
            Color::White => WHITE,
            Color::Red => RED,
            Color::Green => GREEN,
            Color::Blue => BLUE,
            Color::Yellow => YELLOW,
        }
    }
}
