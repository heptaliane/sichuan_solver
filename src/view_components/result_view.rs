use std::collections::HashMap;

use yew::prelude::*;
use yew::Properties;

use super::card::Card;
use super::super::components::{Coord, Tile, Nodes};

#[derive(Properties, PartialEq)]
pub struct ResultViewProps {
    pub rows: usize,
    pub cols: usize,

    pub tiles: HashMap<Coord, Tile>,
    pub connections: Vec<Nodes>,
}

#[function_component(ResultView)]
pub fn result_view(props: &ResultViewProps) -> Html {
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

