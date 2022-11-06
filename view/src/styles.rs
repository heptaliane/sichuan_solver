use once_cell::sync::OnceCell;
use std::collections::HashMap;
use stylist::Style;

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
const TILE_IMAGE_BUTTON_STYLE_PROPS: [(&'static str, &'static str); 7] = [
    ("color", "black"),
    ("background-color", "white"),
    ("border", "solid 1px"),
    ("border-radius", "5px"),
    ("margin", "auto"),
    ("width", "fit-content"),
    ("height", "fit-content"),
];
const CENTERING_CONTAINER_STYLE_PROPS: [(&'static str, &'static str); 3] = [
    ("padding", "5px"),
    ("margin", "auto"),
    ("text-align", "center"),
];
const FLEX_CONTAINER_STYLE_PROPS: [(&'static str, &'static str); 3] = [
    ("display", "flex"),
    ("flex-direction", "row"),
    ("flex-wrap", "wrap"),
];
const FLEX_ITEM_STYLE_PROPS: [(&'static str, &'static str); 2] =
    [("width", "fit-content"), ("flex", "auto")];
const PANEL_CONTAINER_STYLE_PROPS: [(&'static str, &'static str); 10] = [
    ("position", "relative"),
    ("text-align", "center"),
    ("display", "flex"),
    ("flex-direction", "column"),
    ("min-width", "0"),
    ("word-wrap", "break-word"),
    ("background-clip", "border-box"),
    ("background-color", "#fff"),
    ("border", "1px solid"),
    ("border-radius", ".25rem"),
];
const PANEL_HEADER_STYLE_PROPS: [(&'static str, &'static str); 3] = [
    ("padding", ".5rem 1rem"),
    ("margin-bottom", "0"),
    ("border-bottom", "1px solid"),
];
const PANEL_BODY_STYLE_PROPS: [(&'static str, &'static str); 2] =
    [("flex", "1 1 auto"), ("padding", "1rem 1rem")];
const PANEL_FOOTER_STYLE_PROPS: [(&'static str, &'static str); 2] =
    [("padding", ".5rem 1rem"), ("border-top", "1px solid")];
const CLICKABLE_STYLE_PROPS: [(&'static str, &'static str); 1] = [("cursor", "pointer")];

pub static ACTIVE_TILE_HIGHLIGHT_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static INACTIVE_TILE_HIGHLIGHT_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static TILE_IMAGE_BUTTON_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static CENTERING_CONTAINER_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static FLEX_CONTAINER_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static FLEX_ITEM_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static PANEL_CONTAINER_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static PANEL_HEADER_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static PANEL_BODY_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static PANEL_FOOTER_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static CLICKABLE_STYLE: OnceCell<ComponentStyle> = OnceCell::new();

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentStyle {
    pub props: HashMap<&'static str, &'static str>,
}

impl ComponentStyle {
    pub fn css(&self, extras: Option<HashMap<&str, &str>>) -> Style {
        let css_str = self.create_css_str(&self.props);

        match extras {
            Some(props) => {
                let extra_css = self.create_css_str(&props);
                Style::new([css_str, extra_css].join(""))
            }
            None => Style::new(css_str),
        }
        .expect("Invalid CSS")
    }

    fn create_css_str(&self, props: &HashMap<&str, &str>) -> String {
        props
            .iter()
            .map(|(k, v)| format!("{}: {};", k, v))
            .collect::<Vec<String>>()
            .join("")
    }
}

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
    pub fn value(&self) -> Option<&ComponentStyle> {
        match self {
            TileHighlightStyle::ACTIVE => ACTIVE_TILE_HIGHLIGHT_STYLE.get(),
            TileHighlightStyle::INACTIVE => INACTIVE_TILE_HIGHLIGHT_STYLE.get(),
        }
    }
}

pub fn init() {
    [
        (
            &ACTIVE_TILE_HIGHLIGHT_STYLE,
            ACTIVE_TILE_HIGHLIGHT_STYLE_PROPS.to_vec(),
        ),
        (
            &INACTIVE_TILE_HIGHLIGHT_STYLE,
            INACTIVE_TILE_HIGHLIGHT_STYLE_PROPS.to_vec(),
        ),
        (
            &TILE_IMAGE_BUTTON_STYLE,
            TILE_IMAGE_BUTTON_STYLE_PROPS.to_vec(),
        ),
        (
            &CENTERING_CONTAINER_STYLE,
            CENTERING_CONTAINER_STYLE_PROPS.to_vec(),
        ),
        (&FLEX_ITEM_STYLE, FLEX_ITEM_STYLE_PROPS.to_vec()),
        (&FLEX_CONTAINER_STYLE, FLEX_CONTAINER_STYLE_PROPS.to_vec()),
        (&PANEL_CONTAINER_STYLE, PANEL_CONTAINER_STYLE_PROPS.to_vec()),
        (&PANEL_HEADER_STYLE, PANEL_HEADER_STYLE_PROPS.to_vec()),
        (&PANEL_BODY_STYLE, PANEL_BODY_STYLE_PROPS.to_vec()),
        (&PANEL_FOOTER_STYLE, PANEL_FOOTER_STYLE_PROPS.to_vec()),
        (&CLICKABLE_STYLE, CLICKABLE_STYLE_PROPS.to_vec()),
    ]
    .iter()
    .for_each(|(cell, props)| {
        let result = cell.set(ComponentStyle {
            props: HashMap::from_iter(props.clone().into_iter()),
        });

        match result {
            Err(err) => log::warn!("Failed to load css: {:?}", err),
            _ => (),
        }
    });
}
