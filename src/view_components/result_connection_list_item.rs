use std::rc::Rc;

use web_sys::{Element, HtmlImageElement, Node};
use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::super::components::Coord;

#[derive(Properties, PartialEq)]
pub struct ResultConnectionListItemProps {
    pub selected: bool,
    pub id: usize,
    pub coords: [Coord; 2],
    pub image: Option<Rc<HtmlImageElement>>,

    pub onclick: Callback<Option<usize>>,
}

#[function_component(ResultConnectionListItem)]
pub fn result_connection_list_item(props: &ResultConnectionListItemProps) -> Html {
    let node: Option<Node> = match props.image {
        Some(ref img) => Some(Element::from(img.as_ref().clone()).into()),
        _ => None,
    };
    let onclick = props.onclick.clone();
    let next_id = match props.selected {
        true => None,
        false => Some(props.id),
    };

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
                                {
                                    match node {
                                        Some(img) => Html::VRef(img),
                                        _ => html!{},
                                    }
                                }
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
