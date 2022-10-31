use std::collections::HashMap;
use web_sys::HtmlInputElement;
use yew::callback::Callback;
use yew::prelude::*;
use yew::{NodeRef, Properties};

use super::styles;
use super::tile_map_view::TileMapViewModel;
use super::tile_selector::TileSelectorModel;

const DEFAULT_MAP_ROWS: usize = 6;
const DEFAULT_MAP_COLS: usize = 5;
const MIN_MAP_SIZE: usize = 1;
const MAP_BORDER: &str = "solid 3px #6200EE";
const SELECTOR_BORDER: &str = "solid 3px #6200EE";

pub enum TileEditViewMsg {
    TileClicked([usize; 2]),
    TileSelected(usize),
    MapSizeChanged,
}

pub struct TileEditViewModel {
    row_ref: NodeRef,
    col_ref: NodeRef,
    active: HashMap<[usize; 2], styles::TileHighlightStyle>,
    selected: Option<usize>,
}

#[derive(Properties, PartialEq)]
pub struct TileEditViewProps {
    #[prop_or(HashMap::new())]
    pub tile_map: HashMap<[usize; 2], usize>,

    pub onupdate: Callback<([usize; 2], Option<usize>)>,
}

impl Component for TileEditViewModel {
    type Message = TileEditViewMsg;
    type Properties = TileEditViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            row_ref: NodeRef::default(),
            col_ref: NodeRef::default(),
            active: HashMap::new(),
            selected: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::TileClicked(coord) => {
                self.active = HashMap::from([(coord, styles::TileHighlightStyle::ACTIVE)]);
                ctx.props().onupdate.emit((coord, self.selected));
            }
            Self::Message::TileSelected(idx) => {
                self.selected = match self.selected {
                    Some(current) if idx == current => None,
                    _ => Some(idx),
                };
                self.active = HashMap::new();
                return true;
            }
            Self::Message::MapSizeChanged => {
                return true;
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let onclick = ctx
            .link()
            .callback(|coord: [usize; 2]| TileEditViewMsg::TileClicked(coord));
        let onselect = ctx
            .link()
            .callback(|idx: usize| TileEditViewMsg::TileSelected(idx));
        let onresize = ctx.link().callback(|_| TileEditViewMsg::MapSizeChanged);

        html! {
            <div>
                <div
                    class={
                        styles::PANEL_CONTAINER_STYLE
                            .get()
                            .unwrap()
                            .css(Some(HashMap::from([("border", SELECTOR_BORDER)])))
                    }
                >
                    <div>
                        <div>
                            <label>{"Rows: "}</label>
                            <input
                                type="number"
                                ref={&self.row_ref}
                                min={MIN_MAP_SIZE.to_string()}
                                oninput={onresize.clone()}
                            />
                        </div>
                        <div>
                            <label>{"Cols: "}</label>
                            <input
                                type="number"
                                ref={&self.col_ref}
                                min={MIN_MAP_SIZE.to_string()}
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
                <div
                    class={
                        styles::PANEL_CONTAINER_STYLE
                            .get()
                            .unwrap()
                            .css(Some(HashMap::from([("border", MAP_BORDER)])))
                    }
                >
                    <TileMapViewModel
                        rows={self.row()}
                        cols={self.col()}
                        active={self.active.clone()}
                        tile_map={props.tile_map.clone()}
                        onclick={onclick}
                    >
                    </TileMapViewModel>
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
}
