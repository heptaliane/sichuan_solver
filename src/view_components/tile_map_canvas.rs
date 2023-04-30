use std::collections::HashMap;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use yew::callback::Callback;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::{NodeRef, Properties};

use super::super::components::{Coord, CoordElement, Tile, TileMap};
use super::icons::tiles::{create_all_tiles, AsyncTileImage};

const TILE_WIDTH: usize = 80;
const TILE_HEIGHT: usize = 100;
const MAP_MARGIN: usize = 5;
const DEFAULT_BG_COLOR: &str = "white";
const BLANK_BG_COLOR: &str = "gainsboro";
const GRID_COLOR: &str = "gray";
const GRID_WIDTH: f64 = 1.0;

pub enum TileMapCanvasMsg {
    TileClicked([i32; 2]),
    ImageLoaded(Vec<AsyncTileImage>),
}

#[derive(Properties, PartialEq)]
pub struct TileMapCanvas {
    images: Vec<AsyncTileImage>,
    canvas: NodeRef,
}

#[derive(Properties, PartialEq)]
pub struct TileMapCanvasProps {
    pub rows: usize,
    pub cols: usize,

    #[prop_or(TILE_WIDTH)]
    pub tile_width: usize,
    #[prop_or(TILE_HEIGHT)]
    pub tile_height: usize,
    #[prop_or(MAP_MARGIN)]
    pub map_margin: usize,

    pub tiles: TileMap,

    #[prop_or(HashMap::new())]
    pub bg_color: HashMap<Coord, AttrValue>,

    pub onclick: Callback<Coord>,
}

impl Component for TileMapCanvas {
    type Message = TileMapCanvasMsg;
    type Properties = TileMapCanvasProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            images: Vec::new(),
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::TileClicked([x, y]) => match (self.xpos(ctx, x), self.ypos(ctx, y)) {
                (Some(xpos), Some(ypos)) => {
                    ctx.props().onclick.emit([xpos, ypos]);
                    true
                }
                _ => false,
            },
            Self::Message::ImageLoaded(provider) => {
                self.images = provider;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx
            .link()
            .callback(|e: MouseEvent| Self::Message::TileClicked([e.offset_x(), e.offset_y()]));

        html! {
            <canvas
                ref={self.canvas.to_owned()}
                height={self.map_height(ctx).to_string()}
                width={self.map_width(ctx).to_string()}
                onclick={onclick}
            />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let onload = ctx
                .link()
                .callback(|provider| Self::Message::ImageLoaded(provider));
            spawn_local(async move {
                let fetched_images = create_all_tiles().await;
                onload.emit(fetched_images);
            });
        } else {
            self.reset_map(ctx);
            self.draw_backgrounds(ctx);
            self.draw_images(ctx);
        }
    }
}

impl TileMapCanvas {
    fn canvas_context(&self) -> CanvasRenderingContext2d {
        self.canvas
            .cast::<HtmlCanvasElement>()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap()
    }

    fn map_height(&self, ctx: &Context<Self>) -> f64 {
        (ctx.props().tile_height * ctx.props().rows + ctx.props().map_margin * 2) as f64
    }

    fn map_width(&self, ctx: &Context<Self>) -> f64 {
        (ctx.props().tile_width * ctx.props().cols + ctx.props().map_margin * 2) as f64
    }

    fn tile_top(&self, ctx: &Context<Self>, i: CoordElement) -> f64 {
        (ctx.props().tile_height * (i as usize) + ctx.props().map_margin) as f64
    }

    fn tile_left(&self, ctx: &Context<Self>, i: CoordElement) -> f64 {
        (ctx.props().tile_width * (i as usize) + ctx.props().map_margin) as f64
    }

    fn xpos(&self, ctx: &Context<Self>, xpixel: i32) -> Option<CoordElement> {
        let cols = ctx.props().cols as i32;
        match (xpixel as f64 / ctx.props().tile_width as f64).floor() as i32 {
            xpos if xpos >= 0 && xpos < cols => Some(xpos as CoordElement),
            _ => None,
        }
    }

    fn ypos(&self, ctx: &Context<Self>, ypixel: i32) -> Option<CoordElement> {
        let rows = ctx.props().rows as i32;
        match (ypixel as f64 / ctx.props().tile_height as f64).floor() as i32 {
            ypos if ypos >= 0 && ypos < rows => Some(ypos as CoordElement),
            _ => None,
        }
    }

    fn reset_map(&self, ctx: &Context<Self>) {
        let context = self.canvas_context();
        context.set_fill_style(&JsValue::from_str(BLANK_BG_COLOR));
        context.fill_rect(0., 0., self.map_width(ctx), self.map_height(ctx));

        let (w, h) = (
            ctx.props().tile_width as f64,
            ctx.props().tile_height as f64,
        );
        for (&[xpos, ypos], _) in &ctx.props().tiles {
            let (x, y) = (self.tile_left(ctx, xpos), self.tile_top(ctx, ypos));
            self.draw_background(&context, x, y, w, h, DEFAULT_BG_COLOR);
        }

        let (rows, cols) = (ctx.props().rows, ctx.props().cols);
        context.set_stroke_style(&JsValue::from_str(GRID_COLOR));
        context.set_line_width(GRID_WIDTH);
        for i in 0..=cols {
            context.move_to(self.tile_left(ctx, i), self.tile_top(ctx, 0));
            context.line_to(self.tile_left(ctx, i), self.tile_top(ctx, rows));
            context.stroke();
        }
        for i in 0..=rows {
            context.move_to(self.tile_left(ctx, 0), self.tile_top(ctx, i));
            context.line_to(self.tile_left(ctx, cols), self.tile_top(ctx, i));
            context.stroke();
        }
    }

    fn draw_background(
        &self,
        ctx: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        color: &str,
    ) {
        ctx.set_fill_style(&JsValue::from_str(&color));
        ctx.fill_rect(x, y, w, h);
    }

    fn draw_image(
        &self,
        ctx: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        tile: Tile,
    ) {
        if let Some(img) = self.images.get(tile as usize) {
            let result =
                ctx.draw_image_with_html_image_element_and_dw_and_dh(img.as_ref(), x, y, w, h);
            match result {
                Err(_) => log::info!("Failed to load image."),
                _ => (),
            }
        }
    }

    fn draw_backgrounds(&self, ctx: &Context<Self>) {
        let (w, h) = (
            ctx.props().tile_width as f64,
            ctx.props().tile_height as f64,
        );
        let context = self.canvas_context();

        for (&[xpos, ypos], color) in &ctx.props().bg_color {
            let (x, y) = (self.tile_left(ctx, xpos), self.tile_top(ctx, ypos));
            self.draw_background(&context, x, y, w, h, color);
        }
    }

    fn draw_images(&self, ctx: &Context<Self>) {
        let context = self.canvas_context();
        let (w, h) = (
            ctx.props().tile_width as f64,
            ctx.props().tile_height as f64,
        );

        for (&[xpos, ypos], &tile) in &ctx.props().tiles {
            let (x, y) = (self.tile_left(ctx, xpos), self.tile_top(ctx, ypos));
            self.draw_image(&context, x, y, w, h, tile);
        }
    }
}
