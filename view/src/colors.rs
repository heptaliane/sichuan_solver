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
            red,
            green,
            blue,
            alpha,
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
    pub foreground: Color,
    pub background: Color,
    pub border: Color,
}

pub static PRIMARY_COLOR: ColorPalette = ColorPalette {
    foreground: Color::new(0xff, 0xff, 0xff, 1.0),
    background: Color::new(0x0d, 0x6e, 0xfd, 1.0),
    border: Color::new(0x0d, 0x6e, 0xfd, 1.0),
};

pub static DEFAULT_COLOR: ColorPalette = ColorPalette {
    foreground: Color::new(0x21, 0x25, 0x29, 1.0),
    background: Color::new(0, 0, 0, 0.03),
    border: Color::new(0, 0, 0, 0.125),
};
