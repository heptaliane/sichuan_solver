use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::super::components::{Coord, TileMap};
use super::tile_map_canvas::TileMapCanvas;

#[derive(Properties, PartialEq)]
pub struct EditMapCanvasProps {
    pub rows: usize,
    pub cols: usize,
    pub tiles: TileMap,

    pub onselect: Callback<Coord>,
}

#[function_component(EditMapCanvas)]
pub fn edit_map_canvas(props: &EditMapCanvasProps) -> Html {
    let onselect = props.onselect.clone();
    html! {
        <div>
            <TileMapCanvas
                rows={props.rows}
                cols={props.cols}
                tiles={props.tiles.to_owned()}
                onclick={onselect}
            />
        </div>
    }
}
