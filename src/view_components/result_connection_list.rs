use wasm_bindgen_futures::spawn_local;
use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::super::components::{Nodes, TileMap};
use super::super::solver::utils::get_node_edges;
use super::icons::tiles::TileImageProvider;
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
    let images = use_state(|| TileImageProvider::blank_new());
    {
        let images_clone = images.to_owned();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let fetched_images = TileImageProvider::new().await;
                    images_clone.set(fetched_images);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <ol class="list-group">
        {props.connections.iter().enumerate().map(|(i, nodes)| {
            let onselect = props.onselect.clone();
            let [c1, c2] = get_node_edges(&nodes);
            let tile = props.tiles[&c1];

            html!{
                <ResultConnectionListItem
                    selected={props.selected == Some(i)}
                    id={i}
                    coords={[c1, c2]}
                    image={images.get(tile)}
                    onclick={onselect}
                />
            }
       }).collect::<Html>()}
        </ol>
    }
}
