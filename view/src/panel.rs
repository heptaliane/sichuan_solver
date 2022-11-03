use std::collections::HashMap;

use yew::{function_component, html, Children, Properties};

use super::colors;
use super::styles;

#[derive(Properties, PartialEq)]
pub struct PanelProps {
    pub children: Children,

    #[prop_or(None)]
    pub header: Option<&'static str>,

    #[prop_or(None)]
    pub footer: Option<&'static str>,

    #[prop_or(&colors::DEFAULT_COLOR)]
    pub color: &'static colors::ColorPalette,
}

#[function_component(Panel)]
pub fn panel(props: &PanelProps) -> Html {
    let border_color: &str = &props.color.border.css_str();
    let fg_color: &str = &props.color.foreground.css_str();
    let bg_color: &str = &props.color.background.css_str();

    html! {
        <div
            class={
                styles::PANEL_CONTAINER_STYLE
                    .get()
                    .unwrap()
                    .css(Some(HashMap::from([
                        ("border-color", border_color),
                    ])))
            }
        >
            if let Some(header_content) = &props.header {
                <div
                    class={
                        styles::PANEL_HEADER_STYLE
                            .get()
                            .unwrap()
                            .css(Some(HashMap::from([
                                ("color", fg_color),
                                ("background-color", bg_color),
                                ("border-bottom-color", border_color),
                            ])))
                    }
                >
                    {header_content}
                </div>
            }
            <div
                class={
                    styles::PANEL_BODY_STYLE
                        .get()
                        .unwrap()
                        .css(None)
                }
            >
                { for props.children.iter() }
            </div>
            if let Some(footer_content) = &props.footer {
                <div
                    class={
                        styles::PANEL_FOOTER_STYLE
                            .get()
                            .unwrap()
                            .css(Some(HashMap::from([
                                ("color", fg_color),
                                ("background-color", bg_color),
                                ("border-top-color", border_color),
                            ])))
                    }
                >
                    {footer_content}
                </div>
            }
        </div>
    }
}
