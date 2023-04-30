use wasm_bindgen::JsCast;
use web_sys::Event;
use web_sys::HtmlInputElement;
use yew::callback::Callback;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct InputFormControlElementProps {
    pub label: AttrValue,
    pub input_type: AttrValue,
    pub value: AttrValue,

    pub onchange: Callback<AttrValue>,
}

#[function_component(InputFormControlElement)]
pub fn input_form_control_element(props: &InputFormControlElementProps) -> Html {
    let onchange = props.onchange.clone();

    html! {
        <div>
            <label class="form-label">
                {&props.label}
            </label>
            <input
                type={props.input_type.clone()}
                value={props.value.clone()}
                onchange={Callback::from(move |e: Event| {
                    let target = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                    if let Some(input) = target {
                        onchange.emit(AttrValue::from(input.value()));
                    }
                })}
            />
        </div>
    }
}
