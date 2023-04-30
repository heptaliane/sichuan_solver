use std::collections::HashMap;

use yew::callback::Callback;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::Properties;

use super::super::components::{Coord, Nodes, TileMap};
use super::super::solver::utils::get_node_edges;
use super::tile_map_canvas::TileMapCanvas;

const TAKEN_TILE_COLOR: &str = "lightgray";
const ACTIVE_TILE_COLOR: &str = "lightyellow";

#[derive(Properties, PartialEq)]
pub struct ResultMapCanvasProps {
    pub rows: usize,
    pub cols: usize,
    pub tiles: TileMap,

    pub connections: Vec<Nodes>,
    pub cursor: usize,

    pub onselect: Callback<usize>,
}

#[function_component(ResultMapCanvas)]
pub fn result_map_canvas(props: &ResultMapCanvasProps) -> Html {
    let onselect = props.onselect.clone();
    let connections = props.connections.clone();
    let handle_select = Callback::from(move |selected| {
        if let Some((idx, _)) = connections
            .iter()
            .map(|node| get_node_edges(&node))
            .enumerate()
            .find(|(_, coords)| coords.iter().find(|&&coord| coord == selected) != None)
        {
            onselect.emit(idx);
        }
    });

    let (taken, _) = props.connections.split_at(props.cursor);
    let mut backgrounds: HashMap<Coord, AttrValue> = taken
        .iter()
        .map(|node| get_node_edges(&node))
        .flatten()
        .map(|coord| (coord, AttrValue::from(TAKEN_TILE_COLOR)))
        .collect();
    for &coord in &props.connections[props.cursor] {
        backgrounds.insert(coord, AttrValue::from(ACTIVE_TILE_COLOR));
    }

    html! {
        <div>
            <TileMapCanvas
                rows={props.rows}
                cols={props.cols}
                tiles={props.tiles.to_owned()}
                bg_color={backgrounds}
                onclick={handle_select}
            />
        </div>
    }
}
