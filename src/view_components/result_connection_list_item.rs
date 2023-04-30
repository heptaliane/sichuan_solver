use wasm_bindgen_futures::spawn_local;
use web_sys::Node;
use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::super::components::{Coord, Tile};
use super::icons::tiles::AsyncTileImage;

#[derive(Properties, PartialEq)]
pub struct ResultConnectionListItemProps {
    pub selected: bool,
    pub id: usize,
    pub coords: [Coord; 2],
    pub tile: Tile,

    pub onclick: Callback<Option<usize>>,
}

#[function_component(ResultConnectionListItem)]
pub fn result_connection_list_item(props: &ResultConnectionListItemProps) -> Html {
    let onclick = props.onclick.clone();
    let next_id = match props.selected {
        true => None,
        false => Some(props.id),
    };

    let image = use_state(|| None::<Node>);
    {
        let image_clone = image.to_owned();
        let tile = props.tile.to_owned();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let fetched_image = AsyncTileImage::new(tile).await;
                    image_clone.set(Some(fetched_image.node()));
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div
            class={
                match props.selected {
                    true => "list-group-item btn active",
                    false => "list-group-item btn",
                }
            }
            onclick={Callback::from(move |_| onclick.emit(next_id))}
        >
            <li class="list-item">
                <div class="container">
                    <div class="row">
                        <div class="col-3">
                            <div class="card bg-light fit-content">
                            {match image.as_ref() {
                                Some(img) => {Html::VRef(img.to_owned())},
                                None => html!{},
                            }}
                            </div>
                        </div>
                        <div class="col-9 left-align">
                            {format!("{:?} -> {:?}", props.coords[0], props.coords[1])}
                        </div>
                    </div>
                </div>
            </li>
        </div>
    }
}
