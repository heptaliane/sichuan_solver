use std::collections::HashMap;
use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::components::Coord;
use super::icon::tile::MahjongTileImage;
use super::solve_step_list_item::SolveStepListItemModel;

const TILE_WIDTH: u32 = 20;
const TILE_HEIGHT: u32 = 25;

pub enum SolveStepListMsg {
    ItemClicked(usize),
}

pub struct SolveStepListModel {
    image_data: MahjongTileImage,
}

#[derive(Properties, PartialEq)]
pub struct SolveStepListProps {
    #[prop_or(None)]
    pub selected: Option<usize>,

    #[prop_or(Vec::new())]
    pub coords: Vec<[Coord; 2]>,
    #[prop_or(HashMap::new())]
    pub tiles: HashMap<Coord, usize>,

    pub onclick: Callback<usize>,
}

impl Component for SolveStepListModel {
    type Message = SolveStepListMsg;
    type Properties = SolveStepListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let mut images = MahjongTileImage::new();
        images.resize(TILE_WIDTH, TILE_HEIGHT);

        Self { image_data: images }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected = ctx.props().selected;
        let tiles = &ctx.props().tiles;

        html! {
            <ol class="list-group">
                {ctx.props().coords.iter().enumerate().map(|(i, coords)| {
                    html! {
                        <SolveStepListItemModel
                            selected={Some(i) == selected}
                            id={i}
                            coords={coords.clone()}
                            image={self.image_data.get(tiles[&coords[0]])}
                            onclick={ctx.link().callback(|id| Self::Message::ItemClicked(id))}
                        />
                    }
                }).collect::<Html>()}
            </ol>
        }
    }
}
