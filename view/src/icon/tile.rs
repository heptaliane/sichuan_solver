use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::{SvgElement, SvgImageElement};

const SVG_ICON_STR: [&str; 34] = [
    include_str!("./svg/1m.svg"),
    include_str!("./svg/2m.svg"),
    include_str!("./svg/3m.svg"),
    include_str!("./svg/4m.svg"),
    include_str!("./svg/5m.svg"),
    include_str!("./svg/6m.svg"),
    include_str!("./svg/7m.svg"),
    include_str!("./svg/8m.svg"),
    include_str!("./svg/9m.svg"),
    include_str!("./svg/1s.svg"),
    include_str!("./svg/2s.svg"),
    include_str!("./svg/3s.svg"),
    include_str!("./svg/4s.svg"),
    include_str!("./svg/5s.svg"),
    include_str!("./svg/6s.svg"),
    include_str!("./svg/7s.svg"),
    include_str!("./svg/8s.svg"),
    include_str!("./svg/9s.svg"),
    include_str!("./svg/1p.svg"),
    include_str!("./svg/2p.svg"),
    include_str!("./svg/3p.svg"),
    include_str!("./svg/4p.svg"),
    include_str!("./svg/5p.svg"),
    include_str!("./svg/6p.svg"),
    include_str!("./svg/7p.svg"),
    include_str!("./svg/8p.svg"),
    include_str!("./svg/9p.svg"),
    include_str!("./svg/east.svg"),
    include_str!("./svg/south.svg"),
    include_str!("./svg/west.svg"),
    include_str!("./svg/north.svg"),
    include_str!("./svg/red.svg"),
    include_str!("./svg/green.svg"),
    include_str!("./svg/white.svg"),
];

fn create_svg_element(svg_str: &str) -> SvgImageElement {
    let svg = document()
        .create_element("svg")
        .unwrap()
        .dyn_into::<SvgElement>()
        .unwrap();
    svg.set_inner_html(svg_str);
    svg.dyn_into::<SvgImageElement>().unwrap()
}

pub struct MahjongTileImage {
    icons: [SvgImageElement; 34],
}

impl MahjongTileImage {
    pub fn new() -> Self {
        Self {
            icons: SVG_ICON_STR.map(|svg| create_svg_element(svg)),
        }
    }

    pub fn get_ref(&self, idx: usize) -> &SvgImageElement {
        &self.icons[idx]
    }
}
