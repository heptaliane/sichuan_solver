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

    pub onchange: Callback<Option<Tile>>,
}

#[function_component(EditViewControlTileSelector)]
pub fn edit_view_control_tile_selector(props: &EditViewControlTileSelectorProps) -> Html {
    let onchange = props.onchange.clone();
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
        <div class="row">
        {
            images.iter().map(|img| {
                Html::VRef(Element::from(img.as_ref().to_owned()).into())
            }).collect::<Html>()
        }
        </div>
    }
}
