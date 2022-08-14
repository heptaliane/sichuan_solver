use web_sys::{Element, HtmlImageElement, Node};
use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

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

    pub image: HtmlImageElement,

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
        let image: Node = Element::from(ctx.props().image.clone()).into();
        let handle_click = ctx.link().callback(|_| Self::Message::TileSelected);
        html! {
            <div
                onclick={handle_click}
            >
                <input
                    type="radio"
                    checked={ctx.props().selected}
                />
                {Html::VRef(image)}
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
