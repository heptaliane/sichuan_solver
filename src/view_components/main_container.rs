use std::collections::HashMap;

use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::super::components::{Coord, Tile};
use super::container_tab::ContainerTab;
use super::edit_view::EditView;

#[derive(PartialEq, Clone, Copy)]
pub enum MainContainerViewTarget {
    EditView,
    ResultView,
}

#[derive(Properties, PartialEq)]
pub struct MainContainerProps {
    #[prop_or(MainContainerViewTarget::EditView)]
    pub target_view: MainContainerViewTarget,

    pub rows: usize,
    pub cols: usize,
    pub tiles: HashMap<Coord, Tile>,

    pub on_tab_change: Callback<MainContainerViewTarget>,
    pub on_rows_change: Callback<usize>,
    pub on_cols_change: Callback<usize>,
    pub on_editor_submit: Callback<HashMap<Coord, Tile>>,
}

#[function_component(MainContainer)]
pub fn main_container(props: &MainContainerProps) -> Html {
    let on_select_edit_tab = props.on_tab_change.clone();
    let on_select_result_tab = props.on_tab_change.clone();
    let on_rows_change = props.on_rows_change.clone();
    let on_cols_change = props.on_cols_change.clone();
    let on_editor_submit = props.on_editor_submit.clone();

    html! {
        <div>
            <ul class="nav nav-tabs" role="tablist">
                <ContainerTab
                    label="Edit"
                    active={props.target_view == MainContainerViewTarget::EditView}
                    onclick={Callback::from(move |_| {
                        on_select_edit_tab.emit(MainContainerViewTarget::EditView)
                    })}
                />
                <ContainerTab
                    label="Result"
                    active={props.target_view == MainContainerViewTarget::ResultView}
                    onclick={Callback::from(move |_| {
                        on_select_result_tab.emit(MainContainerViewTarget::ResultView)
                    })}
                />
            </ul>
            {
                match props.target_view {
                    MainContainerViewTarget::EditView => html! {
                        <div>
                            <EditView
                                cols={props.cols}
                                rows={props.rows}
                                tiles={props.tiles.clone()}
                                onsubmit={Callback::from(move |tiles| {
                                    on_editor_submit.emit(tiles);
                                })}
                                on_rows_change={Callback::from(move |rows| {
                                    on_rows_change.emit(rows);
                                })}
                                on_cols_change={Callback::from(move |cols| {
                                    on_cols_change.emit(cols);
                                })}
                            />
                        </div>
                    },
                    MainContainerViewTarget::ResultView => html! {
                        <div>
                        </div>
                    },
                }
            }
        </div>
    }
}
