use std::collections::HashMap;
use stylist::Style;

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentStyle {
    pub props: HashMap<&'static str, &'static str>,
}

impl ComponentStyle {
    pub fn css(&self) -> Style {
        let css_str: String = self
            .props
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join(";");
        Style::new(css_str).expect("Invalid CSS")
    }
}

const ACTIVE_TILE_HIGHLIGHT_STYLE_PROPS: [(&'static str, &'static str); 3] = [
    ("fg_color", "red"),
    ("bg_color", "lightyellow"),
    ("line_width", "3"),
];
const INACTIVE_TILE_HIGHLIGHT_STYLE_PROPS: [(&'static str, &'static str); 3] = [
    ("fg_color", "gray"),
    ("bg_color", "lightgray"),
    ("line_width", "1"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct HighlightStyle<'style> {
    pub fg_color: &'style str,
    pub bg_color: &'style str,
    pub line_width: f64,
}

#[derive(Clone, PartialEq)]
pub enum TileHighlightStyle {
    ACTIVE,
    INACTIVE,
}

impl TileHighlightStyle {
    pub fn value(&self) -> ComponentStyle {
        match self {
            TileHighlightStyle::ACTIVE => ComponentStyle {
                props: HashMap::from(ACTIVE_TILE_HIGHLIGHT_STYLE_PROPS),
            },
            TileHighlightStyle::INACTIVE => ComponentStyle {
                props: HashMap::from(INACTIVE_TILE_HIGHLIGHT_STYLE_PROPS),
            },
        }
    }
}

#[derive(Clone, PartialEq)]
struct TileImageButtonStyle<'style> {
    fg_color: &'style str,
    bg_color: &'style str,
    line_width: f64,
    padding: f64,
    radius: f64,
}
