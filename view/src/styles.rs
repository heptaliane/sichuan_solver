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
const TILE_IMAGE_BUTTON_STYLE_PROPS: [(&'static str, &'static str); 5] = [
    ("color", "black"),
    ("background-color", "white"),
    ("border", "solid 1px"),
    ("border-radius", "5px"),
    ("margin", "auto"),
];
const TILE_IMAGE_CONTAINER_STYLE_PROPS: [(&'static str, &'static str); 3] = [
    ("padding", "5px"),
    ("margin", "auto"),
    ("text-align", "center"),
];
const TILE_IMAGE_BUTTON_RADIO_STYLE_PROPS: [(&'static str, &'static str); 1] = [("padding", "3px")];
const TILE_SELECTOR_CONTAINER_STYLE_PROPS: [(&'static str, &'static str); 9] = [
    ("color", "#6200EE"),
    ("background", "white"),
    ("border", "solid 3px"),
    ("border-radius", "20px"),
    ("box-shadow", "10px 5px 5px gray"),
    ("display", "grid"),
    ("grid-template-columns", "repeat(9, 1fr)"),
    ("margin", "auto"),
    ("padding", "20px"),
];

pub static ACTIVE_TILE_HIGHLIGHT_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static INACTIVE_TILE_HIGHLIGHT_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static TILE_IMAGE_BUTTON_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static TILE_IMAGE_CONTAINER_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static TILE_IMAGE_BUTTON_RADIO_STYLE: OnceCell<ComponentStyle> = OnceCell::new();
pub static TILE_SELECTOR_CONTAINER_STYLE: OnceCell<ComponentStyle> = OnceCell::new();

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentStyle {
    pub props: HashMap<&'static str, &'static str>,
}

impl ComponentStyle {
    pub fn css(&self) -> Style {
        let css_str: String = self
            .props
            .iter()
            .map(|(k, v)| format!("{}: {};", k, v))
            .collect::<Vec<String>>()
            .join("");
        Style::new(css_str).expect("Invalid CSS")
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
            &TILE_IMAGE_CONTAINER_STYLE,
            TILE_IMAGE_CONTAINER_STYLE_PROPS.to_vec(),
        ),
        (
            &TILE_IMAGE_BUTTON_RADIO_STYLE,
            TILE_IMAGE_BUTTON_RADIO_STYLE_PROPS.to_vec(),
        ),
        (
            &TILE_SELECTOR_CONTAINER_STYLE,
            TILE_SELECTOR_CONTAINER_STYLE_PROPS.to_vec(),
        )
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
