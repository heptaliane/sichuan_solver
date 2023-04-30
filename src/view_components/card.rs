use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or(AttrValue::from("text-dark bg-light"))]
    pub variant: AttrValue,
    #[prop_or(None)]
    pub header: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class={format!("card {}", props.variant)}>
            {match props.header.clone() {
                Some(header) => html!{
                    <div class="card-header">
                        {header}
                    </div>
                },
                None => html!{}
            }}
            <div class="card-body">
                {props.children.clone()}
            </div>
        </div>
    }
}
