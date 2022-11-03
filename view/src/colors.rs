#[derive(PartialEq, Clone, Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: f32,
}

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        Self {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }

    pub fn css_str(&self) -> String {
        format!(
            "rgba({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct ColorPalette {
    pub background: Color,
    pub foreground: Color,
    pub border: Color,
}

pub static PRIMARY_COLOR: ColorPalette = ColorPalette {
    background: Color::new(13, 110, 253, 1.0),
    foreground: Color::new(255, 255, 255, 1.0),
    border: Color::new(13, 110, 253, 1.0),
};

pub static DEFAULT_COLOR: ColorPalette = ColorPalette {
    background: Color::new(0, 0, 0, 0.03),
    foreground: Color::new(33, 37, 41, 1.0),
    border: Color::new(0, 0, 0, 0.125),
};
