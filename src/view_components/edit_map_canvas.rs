use std::collections::HashMap;

use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::super::components::{Coord, Tile};

#[derive(Properties, PartialEq)]
pub struct EditMapCanvasProps {
    pub rows: usize,
    pub cols: usize,
    pub tiles: HashMap<Coord, Tile>,

    pub onselect: Callback<Coord>,
}

#[function_component(EditMapCanvas)]
pub fn edit_map_canvas(props: &EditMapCanvasProps) -> Html {
    html! {
        <div>
        </div>
    }
}
