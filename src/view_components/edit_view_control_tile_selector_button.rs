use std::rc::Rc;

use web_sys::{Element, HtmlImageElement};
use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;
use web_sys::Node;

#[derive(Properties, PartialEq)]
pub struct EditViewControlTileSelectorButtonProps {
    pub img: Rc<HtmlImageElement>,
    pub selected: bool,

    pub onclick: Callback<()>,
}

#[function_component(EditViewControlTileSelectorButton)]
pub fn edit_view_control_tile_selector_button(props: &EditViewControlTileSelectorButtonProps) -> Html {
    let onclick = props.onclick.clone();
    let img = props.img.to_owned();
    let _ = img.set_attribute("class", "fit-content");
    let node: Node = Element::from(img.as_ref().to_owned()).into();
    html! {
        <div
            class={match props.selected {
                true => "card bg-primary clickable",
                false => "card border-light clickable",
            }}
            onclick={Callback::from(move |_| onclick.emit(()))}
        >
            <div class="card bg-light tile-container">
                {Html::VRef(node)}
            </div>
        </div>
    }
}
