use std::collections::HashMap;

use yew::prelude::*;
use yew::Properties;

use super::super::components::{Coord, Nodes, Tile};
use super::card::Card;
use super::result_map_canvas::ResultMapCanvas;
use super::result_connection_list::ResultConnectionList;

#[derive(Properties, PartialEq)]
pub struct ResultViewProps {
    pub rows: usize,
    pub cols: usize,

    pub tiles: HashMap<Coord, Tile>,
    pub connections: Vec<Nodes>,
}

#[function_component(ResultView)]
pub fn result_view(props: &ResultViewProps) -> Html {
    let selected_index = use_state(|| None::<usize>);
    let onselect = {
        let selected_index = selected_index.clone();
        Callback::from(move |idx| selected_index.set(idx))
    };

    html! {
        <div class="row">
            <div class="col-12 col-lg-4">
                <Card header={"Solving steps"}>
                    <ResultConnectionList
                        tiles={props.tiles.to_owned()}
                        connections={props.connections.to_owned()}
                        selected={*selected_index}
                        onselect={onselect.to_owned()}
                    />
                </Card>
            </div>
            <div class="col-12 col-lg-8">
                <Card>
                    <ResultMapCanvas
                        rows={props.rows}
                        cols={props.cols}
                        tiles={props.tiles.to_owned()}
                        connections={props.connections.to_owned()}
                        cursor={*selected_index}
                        onselect={onselect.to_owned()}
                    />
                </Card>
            </div>
        </div>
    }
}
