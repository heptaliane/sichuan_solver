use web_sys::HtmlInputElement;
use yew::callback::Callback;
use yew::prelude::*;
use yew::{NodeRef, Properties};

use super::tile_selector::TileSelectorModel;

pub enum TileEditControllerMsg {
    TileSelected(usize),
    MapSizeChanged,
}

pub struct TileEditControllerModel {
    row_ref: NodeRef,
    col_ref: NodeRef,
}

#[derive(Properties, PartialEq)]
pub struct TileEditControllerProps {
    #[prop_or(None)]
    pub selected: Option<usize>,

    pub rows: usize,
    pub cols: usize,

    pub onselect: Callback<Option<usize>>,
    pub onchange: Callback<[usize; 2]>,
}

fn try_get_input_value(node_ref: &NodeRef) -> Option<usize> {
    match node_ref.cast::<HtmlInputElement>() {
        Some(elem) => match elem.value().parse() {
            Ok(value) => Some(value),
            Err(_) => None,
        },
        _ => None,
    }
}

impl Component for TileEditControllerModel {
    type Message = TileEditControllerMsg;
    type Properties = TileEditControllerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            row_ref: NodeRef::default(),
            col_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::MapSizeChanged => {
                ctx.props().onchange.emit([
                    match try_get_input_value(&self.row_ref) {
                        Some(value) => value,
                        None => ctx.props().rows,
                    },
                    match try_get_input_value(&self.col_ref) {
                        Some(value) => value,
                        None => ctx.props().cols,
                    },
                ]);
            }
            Self::Message::TileSelected(idx) => {
                ctx.props()
                    .onselect
                    .emit(match Some(idx) == ctx.props().selected {
                        true => None,
                        false => Some(idx),
                    });
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onselect = ctx
            .link()
            .callback(|idx: usize| Self::Message::TileSelected(idx));
        let onchange = ctx.link().callback(|_| Self::Message::MapSizeChanged);
        html! {
            <div class="container">
                <div class="row">
                    <div class="col-12 col-md-6">
                        <div class="container">
                            <div class="row">
                                <div class="col-4">
                                    <label>{ "Rows: " }</label>
                                </div>
                                <div class="col-8">
                                    <input
                                        type="number"
                                        ref={&self.row_ref}
                                        value={ctx.props().rows.to_string()}
                                        min="1"
                                        oninput={onchange.clone()}
                                    />
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="col-12 col-md-6">
                        <div class="container">
                            <div class="row">
                                <div class="col-4">
                                    <label>{ "Cols: " }</label>
                                </div>
                                <div class="col-8">
                                    <input
                                        type="number"
                                        ref={&self.col_ref}
                                        value={ctx.props().rows.to_string()}
                                        min="1"
                                        oninput={onchange.clone()}
                                    />
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="col-md-auto">
                        <TileSelectorModel
                            selected={ctx.props().selected}
                            onclick={onselect.clone()}
                        />
                    </div>
                </div>
            </div>
        }
    }
}
