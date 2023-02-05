use std::collections::HashMap;

use web_sys::HtmlInputElement;
use yew::callback::Callback;
use yew::prelude::*;
use yew::{NodeRef, Properties};

use super::components::Coord;
use super::styles::LineStyle;
use super::tile_map_view::TileMapViewModel;
use super::tile_selector::TileSelectorModel;

static ACTIVE_TILE_BG_COLOR: &str = "lightyellow";
static ACTIVE_TILE_FG_COLOR: &str = "red";
static ACTIVE_TILE_LINE_WIDTH: f64 = 3.0;
static DEFAULT_MAP_ROWS: usize = 5;
static DEFAULT_MAP_COLS: usize = 5;

pub enum TileEditViewMsg {
    TileClicked([usize; 2]),
    TileSelected(usize),
    MapSizeChanged,
}

pub struct TileEditViewModel {
    row_ref: NodeRef,
    col_ref: NodeRef,
    tile_map: HashMap<Coord, usize>,
    bg_color: HashMap<Coord, String>,
    grid: HashMap<Coord, LineStyle>,
    selected: Option<usize>,
}

#[derive(Properties, PartialEq)]
pub struct TileEditViewProps {
    pub onsubmit: Callback<HashMap<[usize; 2], usize>>,
}

impl Component for TileEditViewModel {
    type Message = TileEditViewMsg;
    type Properties = TileEditViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            row_ref: NodeRef::default(),
            col_ref: NodeRef::default(),
            tile_map: HashMap::new(),
            bg_color: HashMap::new(),
            grid: HashMap::new(),
            selected: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::TileClicked(coord) => {
                self.activate_tile(&coord);
                if let Some(idx) = self.selected {
                    match self.tile_map.get(&coord) {
                        Some(&current) if idx == current => {
                            self.tile_map.remove(&coord);
                        }
                        _ => {
                            self.tile_map.insert(coord, idx);
                        }
                    }
                }
                return true;
            }
            Self::Message::TileSelected(idx) => {
                self.selected = match self.selected {
                    Some(current) if idx == current => None,
                    _ => Some(idx),
                };
                self.inactive_tile();
                return true;
            }
            Self::Message::MapSizeChanged => {
                return true;
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx
            .link()
            .callback(|coord: [usize; 2]| TileEditViewMsg::TileClicked(coord));
        let onselect = ctx
            .link()
            .callback(|idx: usize| TileEditViewMsg::TileSelected(idx));
        let onresize = ctx.link().callback(|_| TileEditViewMsg::MapSizeChanged);

        html! {
            <div>
                <div class="card border-secondary item-padding">
                    <div class="card-body">
                        <div class="flex-container">
                            <div class="flex-item">
                                <label>{"Rows: "}</label>
                                <input
                                    type="number"
                                    ref={&self.row_ref}
                                    min="1"
                                    oninput={onresize.clone()}
                                />
                            </div>
                            <div class="flex-item">
                                <label>{"Cols: "}</label>
                                <input
                                    type="number"
                                    ref={&self.col_ref}
                                    min="1"
                                    oninput={onresize.clone()}
                                />
                            </div>
                        </div>
                        <TileSelectorModel
                            selected={self.selected}
                            onclick={onselect}
                        >
                        </TileSelectorModel>
                    </div>
                </div>
                <div class="card border-secondary item-padding">
                    <div class="card-header">
                        {"Mahjong tile mapping"}
                    </div>
                    <div class="card-body">
                        <TileMapViewModel
                            rows={self.row()}
                            cols={self.col()}
                            bg_color={self.bg_color.clone()}
                            grid={self.grid.clone()}
                            tile_map={self.tile_map.clone()}
                            onclick={onclick}
                        >
                        </TileMapViewModel>
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_rendered: bool) {
        if first_rendered {
            self.set_row(DEFAULT_MAP_ROWS);
            self.set_col(DEFAULT_MAP_COLS);
        }
    }
}

impl TileEditViewModel {
    fn row(&self) -> usize {
        match self.row_ref.cast::<HtmlInputElement>() {
            Some(row) => row.value().parse().unwrap(),
            _ => DEFAULT_MAP_ROWS,
        }
    }

    pub fn set_row(&self, size: usize) {
        if let Some(row) = self.row_ref.cast::<HtmlInputElement>() {
            row.set_value(&size.to_string());
        }
    }

    fn col(&self) -> usize {
        match self.col_ref.cast::<HtmlInputElement>() {
            Some(col) => col.value().parse().unwrap(),
            _ => DEFAULT_MAP_COLS,
        }
    }

    pub fn set_col(&self, size: usize) {
        if let Some(col) = self.col_ref.cast::<HtmlInputElement>() {
            col.set_value(&size.to_string());
        }
    }

    fn activate_tile(&mut self, coord: &Coord) {
        self.bg_color = HashMap::from([(coord.clone(), ACTIVE_TILE_BG_COLOR.to_string())]);
        self.grid = HashMap::from([(
            coord.clone(),
            LineStyle {
                color: ACTIVE_TILE_FG_COLOR.to_string(),
                width: ACTIVE_TILE_LINE_WIDTH,
            },
        )]);
    }

    fn inactive_tile(&mut self) {
        self.bg_color = HashMap::new();
        self.grid = HashMap::new();
    }
}
