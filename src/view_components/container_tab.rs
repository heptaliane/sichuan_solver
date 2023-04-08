use yew::callback::Callback;
use yew::prelude::*;
use yew::Properties;


#[derive(Properties, PartialEq)]
pub struct ContainerTabProps {
    pub label: String,

    #[prop_or(false)]
    pub active: bool,

    pub onclick: Callback<()>,
}

#[function_component(ContainerTab)]
pub fn container_tab(props: &ContainerTabProps) -> Html {
    let onclick = props.onclick.clone();

    html! { 
        <li class="nav-item" role="presentation">
            <button
                class={
                    match props.active {
                        true => "nav-link active",
                        false => "nav-link",
                    }
                }
                type="button"
                role="tab"
                onclick={Callback::from(move |_| {
                    onclick.emit(());
                })}
            >
                {&props.label}
            </button>
        </li>
    }
}
