use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::super::components::Tile;
use super::icons::tiles::TileImageProvider;

#[derive(Properties, PartialEq)]
pub struct EditViewControlTileSelectorProps {
    pub selected: Option<Tile>,
    pub loaded: bool,

    pub onchange: Callback<Option<Tile>>,
    pub onload: Callback<()>,
}

#[function_component(EditViewControlTileSelector)]
pub fn edit_view_control_tile_selector(props: &EditViewControlTileSelectorProps) -> Html {
    let onchange = props.onchange.clone();
    let onload = props.onload.clone();
    let images = use_state(|| TileImageProvider::blank_new());
    {
        let images_clone = images.to_owned();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let fetched_images = TileImageProvider::new().await;
                    images_clone.set(fetched_images);
                    onload.emit(());
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="row">
        {
            if props.loaded {
                images.iter().map(|img| {
                    Html::VRef(Element::from(img.as_ref().to_owned()).into())
                }).collect::<Html>()
            } else {
                html! {<p>{ "Loading ..." }</p>}
            }
        }
        </div>
    }
}
