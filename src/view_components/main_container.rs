use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;

use super::container_tab::ContainerTab;

#[derive(PartialEq, Clone, Copy)]
pub enum MainContainerViewTarget {
    EditView,
    ResultView,
}

#[derive(Properties, PartialEq)]
pub struct MainContainerProps {
    #[prop_or(MainContainerViewTarget::EditView)]
    pub target_view: MainContainerViewTarget,

    pub on_tab_change: Callback<MainContainerViewTarget>,
}

#[function_component(MainContainer)]
pub fn main_container(props: &MainContainerProps) -> Html {
    let on_select_edit_tab = props.on_tab_change.clone();
    let on_select_result_tab = props.on_tab_change.clone();

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
