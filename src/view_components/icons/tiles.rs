use std::rc::Rc;

use base64;
use futures::future::join_all;
use js_sys::Promise;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Element, HtmlImageElement, Node};

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

async fn create_image_from_svg(svg_str: &str) -> Result<Rc<HtmlImageElement>, JsValue> {
    let img = HtmlImageElement::new()?;
    let b64svg = base64::encode(svg_str);
    img.set_src(&format!("data:image/svg+xml;base64,{}", b64svg));
    let img_rc = Rc::new(img);

    let promise = Promise::new(&mut |resolve, _| {
        let img_clone = Rc::clone(&img_rc);
        let closure = Closure::wrap(Box::new(move || {
            let _ = resolve.call0(&JsValue::null());
        }) as Box<dyn FnMut()>);
        img_clone.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    });

    JsFuture::from(promise).await?;
    Ok(img_rc)
}

#[derive(PartialEq, Clone)]
pub struct AsyncTileImage {
    image: Rc<HtmlImageElement>,
}

impl AsyncTileImage {
    pub async fn new(tile: Tile) -> Self {
        let future = create_image_from_svg(SVG_ICON_STR[tile as usize]);
        let result = future.await;
        Self {
            image: result.unwrap(),
        }
    }

    pub fn as_ref(&self) -> &HtmlImageElement {
        self.image.as_ref()
    }

    pub fn node(self) -> Node {
        let img = Rc::try_unwrap(self.image).unwrap();
        Element::from(img).into()
    }

    pub fn resize(&self, width: usize, height: usize) {
        self.image.as_ref().set_width(width as u32);
        self.image.as_ref().set_height(height as u32);
    }
}

pub async fn create_all_tiles() -> Vec<AsyncTileImage> {
    let promises = (0..N_SVG_ICONS).map(|tile| AsyncTileImage::new(tile as Tile));
    join_all(promises).await
}
