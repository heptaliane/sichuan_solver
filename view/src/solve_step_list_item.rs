use std::rc::Rc;
use web_sys::{Element, HtmlImageElement, Node};
use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::components::Coord;

pub enum SolveStepListItemMsg {
    ItemClicked,
}

pub struct SolveStepListItemModel {}

#[derive(Properties, PartialEq)]
pub struct SolveStepListItemProps {
    #[prop_or(false)]
    pub selected: bool,

    pub id: usize,

    pub coords: [Coord; 2],

    pub image: Rc<HtmlImageElement>,

    pub onclick: Callback<usize>,
}

impl Component for SolveStepListItemModel {
    type Message = SolveStepListItemMsg;
    type Properties = SolveStepListItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ItemClicked => {
                ctx.props().onclick.emit(ctx.props().id);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let image: &HtmlImageElement = ctx.props().image.as_ref();
        let node: Node = Element::from(image.clone()).into();
        let coords = ctx.props().coords;

        html! {
            <div
                class={
                    match ctx.props().selected {
                        true => "list-group-item flex-container btn btn-secondary",
                        false => "list-group-item flex-container btn btn-outline-secondary",
                    }
                }
            >
                <li class="list-item">
                    <div class="flex-container">
                        <div class="flex-item selector-item">
                            <div class="card bg-light">
                                {Html::VRef(node)}
                            </div>
                        </div>
                        <div class="flex-item selector-item">
                            {format!("({}, {}) => ({}, {})", coords[0][0], coords[0][1], coords[1][0], coords[1][1])}
                        </div>
                    </div>
                </li>
            </div>
        }
    }
}
