use std::collections::HashMap;

use wasm_logger;
use yew::prelude::*;

mod components;
mod view_components;

static DEFAULT_ROWS: usize = 5;
static DEFAULT_COLS: usize = 5;

enum AppMsg {
    TabChanged(view_components::main_container::MainContainerViewTarget),
    EditorSubmitted(HashMap<components::Coord, components::Tile>),
}

struct App {
    target_view: view_components::main_container::MainContainerViewTarget,
    rows: usize,
    cols: usize,
    tiles: HashMap<components::Coord, components::Tile>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            target_view: view_components::main_container::MainContainerViewTarget::EditView,
            rows: DEFAULT_ROWS,
            cols: DEFAULT_COLS,
            tiles: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::TabChanged(target) => {
                self.target_view = target;
                true
            },
            Self::Message::EditorSubmitted(tiles) => {
                self.tiles = tiles;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <view_components::main_container::MainContainer
                    target_view={self.target_view}
                    rows={self.rows}
                    cols={self.cols}
                    tiles={self.tiles.clone()}
                    on_tab_change={
                        ctx.link().callback(|target| Self::Message::TabChanged(target))
                    }
                    on_editor_submit={
                        ctx.link().callback(|tiles| Self::Message::EditorSubmitted(tiles))
                    }
                />
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
