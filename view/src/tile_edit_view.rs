use std::collections::HashMap;

use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::components::Coord;
use super::styles::{LineStyle, ACTIVE_TILE_BG_COLOR, ACTIVE_TILE_LINE};
use super::tile_edit_controller::TileEditControllerModel;
use super::tile_map_view::TileMapViewModel;

static DEFAULT_MAP_ROWS: usize = 5;
static DEFAULT_MAP_COLS: usize = 5;

pub enum TileEditViewMsg {
    TileClicked([usize; 2]),
    TileSelected(Option<usize>),
    MapSizeChanged([usize; 2]),
}

pub struct TileEditViewModel {
    rows: usize,
    cols: usize,
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
            rows: DEFAULT_MAP_ROWS,
            cols: DEFAULT_MAP_COLS,
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
            }
            Self::Message::TileSelected(selected) => {
                self.selected = selected;
                self.inactive_tile();
            }
            Self::Message::MapSizeChanged([rows, cols]) => {
                self.rows = rows;
                self.cols = cols;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx
            .link()
            .callback(|coord: [usize; 2]| TileEditViewMsg::TileClicked(coord));

        html! {
            <div class="contaner">
                <div class="row">
                    <div class="col-12 col-lg-4">
                        <div class="card border-secondary item-padding">
                            <div class="card-body">
                                <TileEditControllerModel
                                    selected={self.selected}
                                    rows={self.rows}
                                    cols={self.cols}
                                    onselect={ctx.link().callback(|idx| Self::Message::TileSelected(idx))}
                                    onchange={ctx.link().callback(|size| Self::Message::MapSizeChanged(size))}
                                />
                            </div>
                        </div>
                    </div>
                    <div class="col-12 col-lg-8">
                        <div class="card border-secondary item-padding">
                            <div class="card-header">
                                {"Mahjong tile mapping"}
                            </div>
                            <div class="card-body">
                                <TileMapViewModel
                                    rows={self.rows}
                                    cols={self.cols}
                                    bg_color={self.bg_color.clone()}
                                    grid={self.grid.clone()}
                                    tile_map={self.tile_map.clone()}
                                    onclick={onclick}
                                >
                                </TileMapViewModel>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl TileEditViewModel {
    fn activate_tile(&mut self, coord: &Coord) {
        self.bg_color = HashMap::from([(coord.clone(), ACTIVE_TILE_BG_COLOR.to_string())]);
        self.grid = HashMap::from([(
            coord.clone(),
            ACTIVE_TILE_LINE.clone(),
        )]);
    }

    fn inactive_tile(&mut self) {
        self.bg_color = HashMap::new();
        self.grid = HashMap::new();
    }
}
