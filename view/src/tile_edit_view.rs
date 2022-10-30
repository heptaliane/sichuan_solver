use std::collections::HashMap;
use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::styles::TileHighlightStyle;
use super::tile_map_view::{TileMapViewModel, TileMapViewProps};
use super::tile_selector::{TileSelectorModel, TileSelectorProps};

const DEFAULT_MAP_ROWS: usize = 5;
const DEFAULT_MAP_COLS: usize = 5;

pub enum TileEditViewMsg {
    TileClicked([usize; 2]),
    TileSelected(usize),
}

pub struct TileEditViewModel {
    active: HashMap<[usize; 2], TileHighlightStyle>,
    selected: Option<usize>,
}

#[derive(Properties, PartialEq)]
pub struct TileEditViewProps {
    #[prop_or(DEFAULT_MAP_ROWS)]
    pub rows: usize,
    #[prop_or(DEFAULT_MAP_COLS)]
    pub cols: usize,
    #[prop_or(HashMap::new())]
    pub tile_map: HashMap<[usize; 2], usize>,

    pub onupdate: Callback<([usize; 2], Option<usize>)>,
}

impl Component for TileEditViewModel {
    type Message = TileEditViewMsg;
    type Properties = TileEditViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            active: HashMap::new(),
            selected: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::TileClicked(coord) => {
                self.active = HashMap::from([(coord, TileHighlightStyle::ACTIVE)]);
                ctx.props().onupdate.emit((coord, self.selected));
            }
            Self::Message::TileSelected(idx) => {
                self.selected = match self.selected {
                    Some(current) if idx == current => None,
                    _ => Some(idx),
                };
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
        html! {
            <div>
                <TileMapViewModel
                    rows={props.rows}
                    cols={props.cols}
                    active={self.active.clone()}
                    tile_map={props.tile_map.clone()}
                    onclick={onclick}
                >
                </TileMapViewModel>
                <TileSelectorModel
                    selected={self.selected}
                    onclick={onselect}
                >
                </TileSelectorModel>
            </div>
        }
    }
}
