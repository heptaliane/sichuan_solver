use base64;
use std::borrow::Borrow;
use std::rc::Rc;
use std::slice::Iter;
use web_sys::HtmlImageElement;

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

fn create_svg_element(svg_str: &str) -> HtmlImageElement {
    let image = HtmlImageElement::new().unwrap();
    let b64svg = base64::encode(svg_str);
    image.set_src(&format!("data:image/svg+xml;base64,{}", b64svg));
    image
}

pub struct MahjongTileImage {
    icons: [Rc<HtmlImageElement>; SVG_ICON_STR.len()],
}

impl MahjongTileImage {
    pub fn new() -> Self {
        Self {
            icons: SVG_ICON_STR.map(|svg| Rc::new(create_svg_element(svg))),
        }
    }

    pub fn get_ref(&self, idx: usize) -> &Rc<HtmlImageElement> {
        &self.icons[idx]
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.icons.iter().for_each(|img_ref| {
            let img: &HtmlImageElement = img_ref.borrow();
            img.set_width(width);
            img.set_height(height);
        });
    }

    pub fn iter(&self) -> Iter<Rc<HtmlImageElement>> {
        self.icons.iter()
    }
}
