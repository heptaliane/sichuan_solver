use std::collections::HashMap;

use wasm_logger;
use yew::prelude::*;

mod components;
mod solver;
mod view_components;

static DEFAULT_ROWS: usize = 5;
static DEFAULT_COLS: usize = 5;

enum AppMsg {
    TabChanged(view_components::main_container::MainContainerViewTarget),
    RowsChanged(usize),
    ColsChanged(usize),
    EditorSubmitted(HashMap<components::Coord, components::Tile>),
}

struct App {
    target_view: view_components::main_container::MainContainerViewTarget,
    rows: usize,
    cols: usize,
    tiles: HashMap<components::Coord, components::Tile>,
    connections: Vec<components::Nodes>,
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
            connections: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::TabChanged(target) => {
                self.target_view = target;
                true
            }
            Self::Message::RowsChanged(rows) => {
                self.rows = rows;
                true
            }
            Self::Message::ColsChanged(cols) => {
                self.cols = cols;
                true
            }
            Self::Message::EditorSubmitted(tiles) => {
                self.tiles = tiles;
                let mut sichuan_solver = solver::solver::SichuanSolver::new(&self.tiles);
                if sichuan_solver.solve().is_ok() { 
                    self.connections = sichuan_solver.result();
                    self.target_view = view_components::main_container::MainContainerViewTarget::ResultView;
                }
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
                    connections={self.connections.to_owned()}
                    on_tab_change={
                        ctx.link().callback(|target| Self::Message::TabChanged(target))
                    }
                    on_editor_submit={
                        ctx.link().callback(|tiles| Self::Message::EditorSubmitted(tiles))
                    }
                    on_rows_change={
                        ctx.link().callback(|rows| Self::Message::RowsChanged(rows))
                    }
                    on_cols_change={
                        ctx.link().callback(|cols| Self::Message::ColsChanged(cols))
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
