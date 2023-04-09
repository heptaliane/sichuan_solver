use std::future::Future;
use std::rc::Rc;
use std::task::Poll;

use base64;
use futures::future::join_all;
use web_sys::HtmlImageElement;

use super::super::super::components::Tile;

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

struct ImageLoader {
    img: Rc<HtmlImageElement>,
}

impl Future for ImageLoader {
    type Output = Rc<HtmlImageElement>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.img.complete() {
            true => Poll::Ready(self.img.to_owned()),
            false => Poll::Pending,
        }
    }
}

impl ImageLoader {
    fn from_svg(svg_str: &str) -> Self {
        let img = HtmlImageElement::new().unwrap();
        let b64svg = base64::encode(svg_str);
        img.set_src(&format!("data:image/svg+xml;base64,{}", b64svg));
        Self { img: Rc::new(img) }
    }
}

#[derive(Clone)]
pub struct TileImageProvider {
    tiles: Vec<Rc<HtmlImageElement>>,
}

impl TileImageProvider {
    pub fn blank_new() -> Self {
        Self { tiles: Vec::new() }
    }

    pub async fn new() -> Self {
        let futures = SVG_ICON_STR.iter().map(|svg| ImageLoader::from_svg(svg));
        let results = join_all(futures).await;
        let mut tiles = Vec::new();
        for result in results {
            tiles.push(result);
        }
        log::info!("Finished");

        Self { tiles }
    }

    pub fn get(&self, tile: Tile) -> &Rc<HtmlImageElement> {
        &self.tiles[tile as usize]
    }

    pub fn iter(&self) -> std::slice::Iter<Rc<HtmlImageElement>> {
        self.tiles.iter()
    }
}
