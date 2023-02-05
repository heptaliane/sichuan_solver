use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use yew::callback::Callback;
use yew::prelude::*;
use yew::{NodeRef, Properties};

use super::icon::tile::MahjongTileImage;
use super::styles::LineStyle;
use super::components::{CoordElement, Coord};

const DEFAULT_TILE_WIDTH: usize = 80;
const DEFAULT_TILE_HEIGHT: usize = 100;
const DEFAULT_MAP_ROWS: usize = 5;
const DEFAULT_MAP_COLS: usize = 5;
const DEFAULT_BG_COLOR: &str = "white";
const DEFAULT_FG_COLOR: &str = "lightgray";
const DEFAULT_GRID_WIDTH: f64 = 1.0;
const DEFAULT_MAP_MARGIN: usize = 5;

pub enum TileMapViewMsg {
    TileClicked([i32; 2]),
}

pub struct TileMapViewModel {
    image_data: MahjongTileImage,
    canvas: NodeRef,
}

#[derive(Properties, PartialEq)]
pub struct TileMapViewProps {
    #[prop_or(DEFAULT_MAP_ROWS)]
    pub rows: usize,
    #[prop_or(DEFAULT_MAP_COLS)]
    pub cols: usize,
    #[prop_or(DEFAULT_TILE_HEIGHT)]
    pub height: usize,
    #[prop_or(DEFAULT_TILE_WIDTH)]
    pub width: usize,
    #[prop_or(DEFAULT_MAP_MARGIN)]
    pub margin: usize,

    #[prop_or(HashMap::new())]
    pub tile_map: HashMap<Coord, usize>,
    #[prop_or(HashMap::new())]
    pub bg_color: HashMap<Coord, String>,
    #[prop_or(HashMap::new())]
    pub grid: HashMap<Coord, LineStyle>,

    pub onclick: Callback<Coord>,
}

impl Component for TileMapViewModel {
    type Message = TileMapViewMsg;
    type Properties = TileMapViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            image_data: MahjongTileImage::new(),
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TileMapViewMsg::TileClicked([x, y]) => match (self.xpos(ctx, x), self.ypos(ctx, y)) {
                (Some(xpos), Some(ypos)) => ctx.props().onclick.emit([xpos, ypos]),
                _ => (),
            },
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
        self.reset_map(ctx);
        self.draw_backgrounds(ctx);
        self.draw_images(ctx);
        self.draw_grids(ctx);
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
        (ctx.props().height * ctx.props().rows + ctx.props().margin * 2) as f64
    }

    fn map_width(&self, ctx: &Context<Self>) -> f64 {
        (ctx.props().width * ctx.props().cols + ctx.props().margin * 2) as f64
    }

    fn tile_top(&self, ctx: &Context<Self>, i: CoordElement) -> f64 {
        (ctx.props().height * (i as usize) + ctx.props().margin) as f64
    }

    fn tile_left(&self, ctx: &Context<Self>, i: CoordElement) -> f64 {
        (ctx.props().width * (i as usize) + ctx.props().margin) as f64
    }

    fn xpos(&self, ctx: &Context<Self>, xpixel: i32) -> Option<CoordElement> {
        let cols = ctx.props().cols as i32;
        match (xpixel as f64 / ctx.props().width as f64).floor() as i32 {
            xpos if xpos >= 0 && xpos < cols => Some(xpos as CoordElement),
            _ => None,
        }
    }

    fn ypos(&self, ctx: &Context<Self>, ypixel: i32) -> Option<CoordElement> {
        let rows = ctx.props().rows as i32;
        match (ypixel as f64 / ctx.props().height as f64).floor() as i32 {
            ypos if ypos >= 0 && ypos < rows => Some(ypos as CoordElement),
            _ => None,
        }
    }

    fn reset_map(&self, ctx: &Context<Self>) {
        let context = self.canvas_context();
        context.set_fill_style(&JsValue::from_str(DEFAULT_BG_COLOR));
        context.fill_rect(0., 0., self.map_width(ctx), self.map_height(ctx));

        let (rows, cols) = (ctx.props().rows, ctx.props().cols);
        context.set_stroke_style(&JsValue::from_str(DEFAULT_FG_COLOR));
        context.set_line_width(DEFAULT_GRID_WIDTH as f64);
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
        idx: usize,
    ) {
        let result = ctx.draw_image_with_html_image_element_and_dw_and_dh(
            self.image_data.get(idx).as_ref(),
            x,
            y,
            w,
            h,
        );
        match result {
            Err(_) => log::info!("Failed to load image."),
            _ => (),
        }
    }

    fn draw_grid(
        &self,
        ctx: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        color: &str,
        line_width: f64,
    ) {
        ctx.set_stroke_style(&JsValue::from_str(color));
        ctx.set_line_width(line_width);
        ctx.stroke_rect(x, y, w, h);
    }

    fn draw_backgrounds(&self, ctx: &Context<Self>) {
        let (w, h) = (ctx.props().width as f64, ctx.props().height as f64);
        let context = self.canvas_context();

        for (&[xpos, ypos], color) in &ctx.props().bg_color {
            let (x, y) = (self.tile_left(ctx, xpos), self.tile_top(ctx, ypos));
            self.draw_background(&context, x, y, w, h, color);
        }
    }

    fn draw_images(&self, ctx: &Context<Self>) {
        let context = self.canvas_context();
        let (w, h) = (ctx.props().width as f64, ctx.props().height as f64);

        for (&[xpos, ypos], &idx) in &ctx.props().tile_map {
            let (x, y) = (self.tile_left(ctx, xpos), self.tile_top(ctx, ypos));
            self.draw_image(&context, x, y, w, h, idx);
        }
    }

    fn draw_grids(&self, ctx: &Context<Self>) {
        let context = self.canvas_context();
        let (w, h) = (ctx.props().width as f64, ctx.props().height as f64);
        let mut grids: Vec<(&[usize; 2], &LineStyle)> = ctx.props().grid.iter().collect();
        grids.sort_by(|(_, l1), (_, l2)| l2.width.partial_cmp(&l1.width).unwrap());

        for (&[xpos, ypos], line) in grids.iter() {
            let (x, y) = (self.tile_left(ctx, xpos), self.tile_top(ctx, ypos));
            self.draw_grid(&context, x, y, w, h, &line.color, line.width);
        }
    }
}
