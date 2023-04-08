use std::collections::HashMap;

use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::super::components::{Coord, Tile};
use super::card::Card;

pub enum EditViewMsg {}

#[derive(Properties, PartialEq)]
pub struct EditViewProps {
    pub cols: usize,
    pub rows: usize,

    pub tiles: HashMap<Coord, Tile>,

    pub onsubmit: Callback<HashMap<Coord, Tile>>,
}

pub struct EditView {
    pub tiles: HashMap<Coord, Tile>,
}

impl Component for EditView {
    type Message = EditViewMsg;
    type Properties = EditViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            tiles: ctx.props().tiles.clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="row">
                <div class="col-12 col-lg-4">
                    <Card>
                    </Card>
                </div>
                <div class="col-12 col-lg-8">
                    <Card>
                    </Card>
                </div>
            </div>
        }
    }
}
