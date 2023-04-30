use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::super::components::{Nodes, TileMap};
use super::super::solver::utils::get_node_edges;
use super::result_connection_list_item::ResultConnectionListItem;

#[derive(Properties, PartialEq)]
pub struct ResultConnectionListProps {
    pub tiles: TileMap,
    pub connections: Vec<Nodes>,
    pub selected: Option<usize>,

    pub onselect: Callback<Option<usize>>,
}

#[function_component(ResultConnectionList)]
pub fn result_connection_list(props: &ResultConnectionListProps) -> Html {
    html! {
        <ol class="list-group">
        {props.connections.iter().enumerate().map(|(i, nodes)| {
            let onselect = props.onselect.clone();
            let [c1, c2] = get_node_edges(&nodes);

            html!{
                <ResultConnectionListItem
                    selected={props.selected == Some(i)}
                    id={i}
                    coords={[c1, c2]}
                    tile={props.tiles[&c1]}
                    onclick={onselect}
                />
            }
       }).collect::<Html>()}
        </ol>
    }
}
