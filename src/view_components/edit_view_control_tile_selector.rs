use wasm_bindgen_futures::spawn_local;
use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::super::components::Tile;
use super::icons::tiles::TileImageProvider;
use super::edit_view_control_tile_selector_button::EditViewControlTileSelectorButton;

#[derive(Properties, PartialEq)]
pub struct EditViewControlTileSelectorProps {
    pub selected: Option<Tile>,

    pub onchange: Callback<Option<Tile>>,
}

#[function_component(EditViewControlTileSelector)]
pub fn edit_view_control_tile_selector(props: &EditViewControlTileSelectorProps) -> Html {
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
            images.iter().enumerate().map(|(i, img)| {
                let onchange = props.onchange.clone();
                let tile = i as Tile;
                let selected = props.selected == Some(tile);
                html! {
                    <div class="col-2">
                        <EditViewControlTileSelectorButton
                            img={img.to_owned()}
                            selected={selected}
                            onclick={Callback::from(move |_| {
                                onchange.emit(match selected { 
                                    true => None,
                                    false => Some(tile),
                                });
                            })}
                        />
                    </div>
                }
            }).collect::<Html>()
        }
        </div>
    }
}
