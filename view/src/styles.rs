#[derive(Clone, Debug, PartialEq)]
pub struct HighlightStyle<'style> {
    pub fg_color: &'style str,
    pub bg_color: &'style str,
    pub line_width: f64,
}

#[derive(Clone, PartialEq)]
pub enum TileHighlightStyle {
    ACTIVE,
}

impl TileHighlightStyle {
    pub fn value(&self) -> HighlightStyle {
        match self {
            TileHighlightStyle::ACTIVE => HighlightStyle {
                fg_color: "red",
                bg_color: "lightyellow",
                line_width: 3.,
            },
        }
    }
}
