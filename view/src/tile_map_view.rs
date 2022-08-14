use std::borrow::Borrow;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use yew::callback::Callback;
use yew::prelude::*;
use yew::{NodeRef, Properties};

use super::icon::tile::MahjongTileImage;
use super::styles::{HighlightStyle, TileHighlightStyle};

const DEFAULT_TILE_WIDTH: usize = 80;
const DEFAULT_TILE_HEIGHT: usize = 100;
const DEFAULT_MAP_ROWS: usize = 5;
const DEFAULT_MAP_COLS: usize = 5;
const GRID_LINE_WIDTH: usize = 1;
const GRID_STYLE: &str = "lightgray";

pub enum TileMapViewMsg {
    TileClicked([i32; 2]),
}

pub struct TileMapViewModel {
    image_data: MahjongTileImage,
    canvas: NodeRef,
    height: usize,
    width: usize,
}

#[derive(Properties, PartialEq)]
pub struct TileMapViewProps {
    #[prop_or(DEFAULT_MAP_ROWS)]
    pub rows: usize,
    #[prop_or(DEFAULT_MAP_COLS)]
    pub cols: usize,
    #[prop_or(HashMap::new())]
    pub active: HashMap<[usize; 2], TileHighlightStyle>,
    #[prop_or(HashMap::new())]
    pub tile_map: HashMap<[usize; 2], usize>,

    pub onclick: Callback<[usize; 2]>,
}

impl Component for TileMapViewModel {
    type Message = TileMapViewMsg;
    type Properties = TileMapViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            image_data: MahjongTileImage::new(),
            canvas: NodeRef::default(),
            height: DEFAULT_TILE_HEIGHT,
            width: DEFAULT_TILE_WIDTH,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TileMapViewMsg::TileClicked([x, y]) => {
                ctx.props().onclick.emit([self.xpos(x), self.ypos(y)]);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx
            .link()
            .callback(|e: MouseEvent| TileMapViewMsg::TileClicked([e.offset_x(), e.offset_y()]));
        html! {
            <canvas
                ref={self.canvas.clone()}
                height={self.map_height(ctx).to_string()}
                width={self.map_width(ctx).to_string()}
                onclick={onclick}
            >
            </canvas>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_rendered: bool) {
        self.clear_canvas(ctx);
        self.draw_grids(ctx);
        for (&[x, y], style) in &ctx.props().active {
            self.highlight_tile(x, y, style.value());
        }
        self.draw_tile_images(ctx);
    }
}

impl TileMapViewModel {
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
        (DEFAULT_TILE_HEIGHT * ctx.props().cols + GRID_LINE_WIDTH * 2) as f64
    }

    fn map_width(&self, ctx: &Context<Self>) -> f64 {
        (DEFAULT_TILE_WIDTH * ctx.props().rows + GRID_LINE_WIDTH * 2) as f64
    }

    fn tile_top(&self, i: usize) -> f64 {
        (self.height * i + GRID_LINE_WIDTH) as f64
    }

    fn tile_left(&self, i: usize) -> f64 {
        (self.width * i + GRID_LINE_WIDTH) as f64
    }

    fn xpos(&self, xpixel: i32) -> usize {
        (xpixel as f64 / self.width as f64).floor() as usize
    }

    fn ypos(&self, ypixel: i32) -> usize {
        (ypixel as f64 / self.height as f64).floor() as usize
    }

    fn clear_canvas(&self, ctx: &Context<Self>) {
        let context = self.canvas_context();

        context.clear_rect(0., 0., self.map_width(ctx), self.map_height(ctx));
    }

    fn draw_grids(&self, ctx: &Context<Self>) {
        let (rows, cols) = (ctx.props().rows, ctx.props().cols);
        let context = self.canvas_context();

        context.set_stroke_style(&JsValue::from_str(GRID_STYLE));
        context.set_line_width(GRID_LINE_WIDTH as f64);
        for i in 0..=cols {
            context.move_to(self.tile_left(i), 0.);
            context.line_to(self.tile_left(i), self.tile_top(rows));
            context.stroke();
        }
        for i in 0..=rows {
            context.move_to(0., self.tile_top(i));
            context.line_to(self.tile_left(cols), self.tile_top(i));
            context.stroke();
        }
    }

    fn highlight_tile(&self, x: usize, y: usize, style: HighlightStyle) {
        let (w, h) = (self.width as f64, self.height as f64);
        let context = self.canvas_context();

        context.set_fill_style(&JsValue::from_str(style.bg_color));
        context.fill_rect(self.tile_left(x), self.tile_top(y), w, h);

        context.set_stroke_style(&JsValue::from_str(style.fg_color));
        context.set_line_width(style.line_width);
        context.stroke_rect(self.tile_left(x), self.tile_top(y), w, h);
    }

    fn draw_tile_images(&self, ctx: &Context<Self>) {
        let (w, h) = (self.width as f64, self.height as f64);
        let context = self.canvas_context();

        for (&[i, j], &idx) in &ctx.props().tile_map {
            let result = context.draw_image_with_html_image_element_and_dw_and_dh(
                self.image_data.get_ref(idx).borrow(),
                self.tile_left(i),
                self.tile_top(j),
                w,
                h,
            );
            match result {
                Err(_) => log::info!("Failed to load image."),
                _ => (),
            }
        }
    }
}
