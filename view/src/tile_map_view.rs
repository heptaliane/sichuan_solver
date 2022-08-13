use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use yew::prelude::*;
use yew::{NodeRef, Properties};

use super::icon::tile::MahjongTileImage;

const DEFAULT_TILE_WIDTH: usize = 100;
const DEFAULT_TILE_HEIGHT: usize = 100;
const DEFAULT_MAP_ROWS: usize = 5;
const DEFAULT_MAP_COLS: usize = 5;
const GRID_LINE_WIDTH: usize = 1;
const GRID_STYLE: &str = "lightgray";
const ACTIVE_LINE_STYLE: &str = "red";
const ACTIVE_LINE_WIDTH: usize = 3;

pub enum TileMapViewMsg {
    TileClicked([i32; 2]),
}

pub struct TileMapViewModel {
    image_data: MahjongTileImage,
    canvas: NodeRef,
    height: usize,
    width: usize,
    active: Option<[i32; 2]>,
}

#[derive(Properties, PartialEq)]
pub struct TileMapViewProps {
    #[prop_or(DEFAULT_MAP_ROWS)]
    pub rows: usize,
    #[prop_or(DEFAULT_MAP_COLS)]
    pub cols: usize,
    pub tile_map: HashMap<(usize, usize), usize>,
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
            active: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TileMapViewMsg::TileClicked([x, y]) => {
                let i: i32 = (x as f64 / self.width as f64).floor() as i32;
                let j: i32 = (y as f64 / self.height as f64).floor() as i32;
                match self.active {
                    Some([current_i, current_j]) if current_i == i && current_j == j => {
                        self.active = None
                    }
                    _ => self.active = Some([i, j]),
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx
            .link()
            .callback(|e: MouseEvent| TileMapViewMsg::TileClicked([e.offset_x(), e.offset_y()]));
        html! {
            <canvas
                ref={self.canvas.clone()}
                height={(DEFAULT_TILE_HEIGHT * ctx.props().rows + GRID_LINE_WIDTH * 2).to_string()}
                width={(DEFAULT_TILE_WIDTH * ctx.props().cols + GRID_LINE_WIDTH * 2).to_string()}
                onclick={onclick}
            >
            </canvas>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_rendered: bool) {
        self.clear_canvas(ctx);
        self.draw_grids(ctx);
        match self.active {
            Some([x, y]) => self.draw_active_tile(x, y),
            _ => (),
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

    fn clear_canvas(&self, ctx: &Context<Self>) {
        let context = self.canvas_context();
        let (rows, cols) = (ctx.props().rows, ctx.props().cols);
        let (w, h) = (self.width, self.height);

        context.clear_rect(
            0.,
            0.,
            (w * cols + GRID_LINE_WIDTH * 2) as f64,
            (h * rows + GRID_LINE_WIDTH * 2) as f64,
        );
    }

    fn draw_grids(&self, ctx: &Context<Self>) {
        let (rows, cols) = (ctx.props().rows, ctx.props().cols);
        let (w, h) = (self.width, self.height);
        let context = self.canvas_context();

        context.set_stroke_style(&JsValue::from_str(GRID_STYLE));
        context.set_line_width(GRID_LINE_WIDTH as f64);
        for i in 0..=cols {
            context.move_to((w * i + GRID_LINE_WIDTH) as f64, 0.);
            context.line_to(
                (w * i + GRID_LINE_WIDTH) as f64,
                (h * rows + GRID_LINE_WIDTH) as f64,
            );
            context.stroke();
        }
        for i in 0..=rows {
            context.move_to(0., (h * i + GRID_LINE_WIDTH) as f64);
            context.line_to(
                (w * cols + GRID_LINE_WIDTH) as f64,
                (h * i + GRID_LINE_WIDTH) as f64,
            );
            context.stroke();
        }
    }

    fn draw_active_tile(&self, x: i32, y: i32) {
        let (w, h) = (self.width, self.height);
        let context = self.canvas_context();

        context.set_stroke_style(&JsValue::from_str(ACTIVE_LINE_STYLE));
        context.set_line_width(ACTIVE_LINE_WIDTH as f64);
        context.stroke_rect(
            (w as i32 * x) as f64,
            (h as i32 * y) as f64,
            w as f64,
            h as f64,
        );
    }

    fn draw_tile_images(&self, ctx: &Context<Self>) {
        let (w, h) = (self.width, self.height);
        let context = self.canvas_context();

        for (&(i, j), &idx) in &ctx.props().tile_map {
            let result = context.draw_image_with_html_image_element_and_dw_and_dh(
                self.image_data.get_ref(idx),
                (w as f64) * (i as f64),
                (h as f64) * (j as f64),
                w as f64,
                h as f64,
            );
            match result {
                Err(e) => log::info!("{}", e.as_string().unwrap_or("None".to_string())),
                _ => (),
            }
        }
    }
}
