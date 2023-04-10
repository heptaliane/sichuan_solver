use std::collections::HashMap;

use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::super::components::{Coord, Tile};
use super::card::Card;
use super::edit_map_canvas::EditMapCanvas;
use super::edit_view_control::EditViewControl;

pub enum EditViewMsg {
    RowsChanged(usize),
    ColsChanged(usize),
    TileChanged(Option<Tile>),
    MapUpdated(Coord),
    MapSubmitted,
}

#[derive(Properties, PartialEq)]
pub struct EditViewProps {
    pub rows: usize,
    pub cols: usize,

    pub tiles: HashMap<Coord, Tile>,

    pub on_rows_change: Callback<usize>,
    pub on_cols_change: Callback<usize>,
    pub onsubmit: Callback<HashMap<Coord, Tile>>,
}

pub struct EditView {
    pub current: Option<Tile>,
    pub tiles: HashMap<Coord, Tile>,
}

impl Component for EditView {
    type Message = EditViewMsg;
    type Properties = EditViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current: None,
            tiles: ctx.props().tiles.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::RowsChanged(rows) => {
                ctx.props().on_rows_change.emit(rows);
                false
            }
            Self::Message::ColsChanged(rows) => {
                ctx.props().on_cols_change.emit(rows);
                false
            }
            Self::Message::TileChanged(tile) => {
                self.current = tile;
                true
            }
            Self::Message::MapUpdated(coord) => {
                match (self.current, self.tiles.get(&coord)) {
                    (Some(next), Some(&current)) if next == current => {
                        self.tiles.remove(&coord);
                    }
                    (Some(next), _) => {
                        self.tiles.insert(coord, next);
                    }
                    (None, _) => {
                        self.tiles.remove(&coord);
                    }
                }
                true
            }
            Self::Message::MapSubmitted => {
                ctx.props().onsubmit.emit(self.tiles.to_owned());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="row">
                <div class="col-12 col-lg-4">
                    <Card>
                        <EditViewControl
                            rows={ctx.props().rows}
                            cols={ctx.props().cols}
                            tile={self.current}
                            submittable={validate_tiles(&self.tiles)}
                            on_rows_change={ctx.link().callback(|rows| Self::Message::RowsChanged(rows))}
                            on_cols_change={ctx.link().callback(|cols| Self::Message::ColsChanged(cols))}
                            on_tile_change={ctx.link().callback(|tile| Self::Message::TileChanged(tile))}
                            onsubmit={ctx.link().callback(|_| Self::Message::MapSubmitted)}
                        />
                    </Card>
                </div>
                <div class="col-12 col-lg-8">
                    <Card>
                        <EditMapCanvas
                            rows={ctx.props().rows}
                            cols={ctx.props().cols}
                            tiles={self.tiles.to_owned()}
                            onselect={ctx.link().callback(|coord| Self::Message::MapUpdated(coord))}
                        />
                    </Card>
                </div>
            </div>
        }
    }
}

fn validate_tiles(tiles: &HashMap<Coord, Tile>) -> bool {
    let mut has_even_tiles: HashMap<&Tile, bool> = HashMap::new();
    for tile in tiles.values() {
        match has_even_tiles.contains_key(tile) {
            true => has_even_tiles.insert(tile, !has_even_tiles[tile]),
            false => has_even_tiles.insert(tile, false),
        };
    }

    has_even_tiles.values().all(|&flg| flg)
}
