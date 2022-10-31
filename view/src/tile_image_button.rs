use std::rc::Rc;
use web_sys::{Element, HtmlImageElement, Node};
use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::styles;

pub enum TileImageButtonMsg {
    TileSelected,
}

pub struct TileImageButtonModel {
    selected: bool,
}

#[derive(Properties, PartialEq)]
pub struct TileImageButtonProps {
    #[prop_or(false)]
    pub selected: bool,

    pub id: usize,

    pub image: Rc<HtmlImageElement>,

    pub onclick: Callback<usize>,
}

impl Component for TileImageButtonModel {
    type Message = TileImageButtonMsg;
    type Properties = TileImageButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            selected: ctx.props().selected,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let image: &HtmlImageElement = ctx.props().image.as_ref();
        let node: Node = Element::from(image.clone()).into();
        let handle_click = ctx.link().callback(|_| Self::Message::TileSelected);
        html! {
            <div
                class={styles::TILE_IMAGE_CONTAINER_STYLE.get().unwrap().css(None)}
                onclick={handle_click}
            >
                <input
                    type="radio"
                    class={styles::TILE_IMAGE_BUTTON_RADIO_STYLE.get().unwrap().css(None)}
                    checked={ctx.props().selected}
                />
                <div class={styles::TILE_IMAGE_BUTTON_STYLE.get().unwrap().css(None)}>
                    {Html::VRef(node)}
                </div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_rendered: bool) {
        self.selected = ctx.props().selected;
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::TileSelected => {
                ctx.props().onclick.emit(ctx.props().id);
            }
        }
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.selected != ctx.props().selected
    }
}
