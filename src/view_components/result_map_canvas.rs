use std::collections::HashMap;

use yew::callback::Callback;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::Properties;

use super::super::components::{Coord, Nodes, TileMap};
use super::super::solver::utils::get_node_edges;
use super::tile_map_canvas::TileMapCanvas;

const TAKEN_TILE_COLOR: &str = "darkgray";
const ACTIVE_TILE_COLOR: &str = "lightyellow";

#[derive(Properties, PartialEq)]
pub struct ResultMapCanvasProps {
    pub rows: usize,
    pub cols: usize,
    pub tiles: TileMap,

    pub connections: Vec<Nodes>,
    pub cursor: Option<usize>,

    pub onselect: Callback<Option<usize>>,
}

#[function_component(ResultMapCanvas)]
pub fn result_map_canvas(props: &ResultMapCanvasProps) -> Html {
    let onselect = props.onselect.clone();
    let connections = props.connections.clone();
    let handle_select = Callback::from(move |selected| {
        match connections
            .iter()
            .map(|node| get_node_edges(&node))
            .enumerate()
            .find(|(_, coords)| coords.iter().find(|&&coord| coord == selected) != None)
        {
            Some((idx, _)) => onselect.emit(Some(idx)),
            _ => onselect.emit(None),
        }
    });

    let mut backgrounds: HashMap<Coord, AttrValue> = HashMap::new();
    let mut current_node: Option<Nodes> = None;
    if let Some(cursor) = props.cursor {
        let (taken, _) = props.connections.split_at(cursor);
        backgrounds.extend(
            taken
                .iter()
                .map(|node| get_node_edges(&node))
                .flatten()
                .map(|coord| (coord, AttrValue::from(TAKEN_TILE_COLOR)))
                .collect::<HashMap<Coord, AttrValue>>(),
        );
        backgrounds.extend(
            get_node_edges(&props.connections[cursor])
                .iter()
                .map(|&coord| (coord, AttrValue::from(ACTIVE_TILE_COLOR)))
                .collect::<HashMap<Coord, AttrValue>>(),
        );
        if let Some(node) = props.connections.get(cursor) {
            current_node = Some(node.clone());
        }
    }

    html! {
        <div>
            <TileMapCanvas
                rows={props.rows}
                cols={props.cols}
                tiles={props.tiles.to_owned()}
                bg_color={backgrounds}
                connection={current_node}
                onclick={handle_select}
            />
        </div>
    }
}
