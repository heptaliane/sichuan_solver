use std::collections::HashMap;

use yew::prelude::*;
use yew::Properties;

use super::components::{Coord, Nodes};
use super::solve_step_list::SolveStepListModel;
use super::tile_map_view::TileMapViewModel;

fn get_node_edge(node: &Nodes) -> [Coord; 2] {
    [
        node.first().unwrap().unwrap(),
        node.iter()
            .reduce(|acc, item| match item {
                Some(_) => item,
                None => acc,
            })
            .unwrap()
            .unwrap(),
    ]
}

pub enum SolveResultViewMsg {
    StepSelected(Option<usize>),
    TileClicked(Coord),
}

pub struct SolveResultViewModel {
    selected: Option<usize>,
}

#[derive(Properties, PartialEq)]
pub struct SolveResultViewProps {
    #[prop_or(HashMap::new())]
    pub tile_map: HashMap<Coord, usize>,
    #[prop_or(Vec::new())]
    pub nodes: Vec<Nodes>,
}

impl Component for SolveResultViewModel {
    type Message = SolveResultViewMsg;
    type Properties = SolveResultViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { selected: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::StepSelected(idx) => {
                self.selected = idx;
            }
            Self::Message::TileClicked(coord) => {
                self.selected = ctx
                    .props()
                    .nodes
                    .iter()
                    .enumerate()
                    .map(|(i, node)| {
                        let [c1, c2] = get_node_edge(node);
                        match coord == c1 || coord == c2 {
                            true => Some(i),
                            false => None,
                        }
                    })
                    .reduce(|acc, i| match i {
                        Some(_) => i,
                        None => acc,
                    }).unwrap_or(None);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="contaier">
                <div class="row">
                    <div class="col-12 col-lg-4">
                        <div class="card border-success item-padding">
                            <div class="card-header">
                                { "Solving steps" }
                            </div>
                            <div class="card-body">
                                <SolveStepListModel
                                    selected={self.selected}
                                    coords={
                                        ctx
                                            .props()
                                            .nodes
                                            .iter()
                                            .map(|node| get_node_edge(node))
                                            .collect::<Vec<[Coord; 2]>>()
                                    }
                                    tiles={ctx.props().tile_map.clone()}
                                    onclick={ctx.link().callback(|idx| Self::Message::StepSelected(idx))}
                                />
                            </div>
                        </div>
                    </div>
                    <div class="col-12 col-lg-8">
                        <div class="card border-success item-padding">
                            <div class="card-header">
                                { "Mahjong tile mapping" }
                            </div>
                            <div class="card-body">
                                <TileMapViewModel
                                    tile_map={ctx.props().tile_map.clone()}
                                    onclick={ctx.link().callback(|coord| Self::Message::TileClicked(coord))}
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
