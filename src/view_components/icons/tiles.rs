use base64;
use std::boxed::Box;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use wasm_bindgen::prelude::Closure;
use web_sys::Event;
use web_sys::HtmlImageElement;

const N_SVG_ICONS: usize = 34;
const SVG_ICON_STR: [&str; N_SVG_ICONS] = [
    include_str!("svg/1m.svg"),
    include_str!("svg/2m.svg"),
    include_str!("svg/3m.svg"),
    include_str!("svg/4m.svg"),
    include_str!("svg/5m.svg"),
    include_str!("svg/6m.svg"),
    include_str!("svg/7m.svg"),
    include_str!("svg/8m.svg"),
    include_str!("svg/9m.svg"),
    include_str!("svg/1s.svg"),
    include_str!("svg/2s.svg"),
    include_str!("svg/3s.svg"),
    include_str!("svg/4s.svg"),
    include_str!("svg/5s.svg"),
    include_str!("svg/6s.svg"),
    include_str!("svg/7s.svg"),
    include_str!("svg/8s.svg"),
    include_str!("svg/9s.svg"),
    include_str!("svg/1p.svg"),
    include_str!("svg/2p.svg"),
    include_str!("svg/3p.svg"),
    include_str!("svg/4p.svg"),
    include_str!("svg/5p.svg"),
    include_str!("svg/6p.svg"),
    include_str!("svg/7p.svg"),
    include_str!("svg/8p.svg"),
    include_str!("svg/9p.svg"),
    include_str!("svg/east.svg"),
    include_str!("svg/south.svg"),
    include_str!("svg/west.svg"),
    include_str!("svg/north.svg"),
    include_str!("svg/red.svg"),
    include_str!("svg/green.svg"),
    include_str!("svg/white.svg"),
];

fn create_svg_element(svg_str: &str) -> HtmlImageElement {
    let image = HtmlImageElement::new().unwrap();
    let b64svg = base64::encode(svg_str);
    image.set_src(&format!("data:image/svg+xml;base64,{}", b64svg));
    image
}

pub struct TileImageProvider {
    tiles: [Rc<HtmlImageElement>; N_SVG_ICONS],
    loaded: [bool; N_SVG_ICONS],
}

impl TileImageProvider {
    pub fn new() -> Self {
        let tiles = SVG_ICON_STR.map(|svg| Rc::new(create_svg_element(svg)));
        let mut loaded = SVG_ICON_STR.map(|_| false);

        tiles.iter().enumerate().for_each(|(i, tile)| {
            let mut loaded_vec = loaded.to_vec();
            let callback = Closure::wrap(Box::new(move |_| {
                loaded_vec[i] = true 
            }) as Box<dyn FnMut(Event)>);
            tile.set_onload(Some(callback.as_ref().unchecked_ref()));
        });

        Self {
            tiles: tiles,
            loaded: loaded,
        }
    }

    pub fn get(&self, idx: usize) -> &Rc<HtmlImageElement> {
        &self.tiles[idx]
    }

    pub fn iter(&self) -> std::slice::Iter<Rc<HtmlImageElement>> {
        self.tiles.iter()
    }

    pub fn loaded(&self) -> bool {
        self.loaded.iter().all(|&value| value)
    }
}
