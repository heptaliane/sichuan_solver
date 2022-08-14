use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::icon::tile::MahjongTileImage;
use super::tile_image_button::TileImageButtonModel;

const TILE_WIDTH: u32 = 40;
const TILE_HEIGHT: u32 = 50;

pub enum TileSelectorMsg {
    TileSelected(usize),
}

pub struct TileSelectorModel {
    image_data: MahjongTileImage,
}

#[derive(Properties, PartialEq)]
pub struct TileSelectorProps {
    #[prop_or(None)]
    pub selected: Option<usize>,

    pub onclick: Callback<usize>,
}

impl Component for TileSelectorModel {
    type Message = TileSelectorMsg;
    type Properties = TileSelectorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let images = MahjongTileImage::new();
        images.resize(TILE_WIDTH, TILE_HEIGHT);

        Self { image_data: images }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::TileSelected(value) => {
                ctx.props().onclick.emit(value);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected = ctx.props().selected;
        html! {
            <div>
                {self.image_data.iter().enumerate().map(|(i, img)| {
                    html! {
                        <TileImageButtonModel
                            image={img.clone()}
                            id={i}
                            selected={Some(i) == selected}
                            onclick={ctx.link().callback(|id| Self::Message::TileSelected(id))}
                        />
                    }
                }).collect::<Html>()}
            </div>
        }
    }
}
