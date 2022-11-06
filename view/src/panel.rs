use std::collections::HashMap;

use yew::{function_component, html, Children, Properties};

use super::colors;
use super::styles;

#[derive(PartialEq)]
pub struct PanelColor<'color> {
    pub font: &'color colors::Color,
    pub body_background: &'color colors::Color,
    pub header_background: &'color colors::Color,
    pub footer_background: &'color colors::Color,
    pub border: &'color colors::Color,
}

const DEFUALT_BODY_BACKGROUND_COLOR: colors::Color = colors::Color::new(0xff, 0xff, 0xff, 1.0);

impl<'color> PanelColor<'color> {
    pub const fn from(palette: &'color colors::ColorPalette) -> Self {
        Self {
            font: &palette.foreground,
            body_background: &DEFUALT_BODY_BACKGROUND_COLOR,
            header_background: &palette.background,
            footer_background: &palette.background,
            border: &palette.border,
        }
    }
}

pub static PRIMARY_PANEL_COLOR: PanelColor = PanelColor::from(&colors::PRIMARY_COLOR);
pub static DEFAULT_PANEL_COLOR: PanelColor = PanelColor::from(&colors::DEFAULT_COLOR);

#[derive(Properties, PartialEq)]
pub struct PanelProps {
    pub children: Children,

    #[prop_or(None)]
    pub header: Option<&'static str>,

    #[prop_or(None)]
    pub footer: Option<&'static str>,

    #[prop_or(&DEFAULT_PANEL_COLOR)]
    pub color: &'static PanelColor<'static>,
}

#[function_component(Panel)]
pub fn panel(props: &PanelProps) -> Html {
    html! {
        <div
            class={
                styles::PANEL_CONTAINER_STYLE
                    .get()
                    .unwrap()
                    .css(Some(HashMap::from([
                        ("border-color", props.color.border.css_str().as_str()),
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
                                ("color", props.color.font.css_str().as_str()),
                                ("background-color", props.color.header_background.css_str().as_str()),
                                ("border-bottom-color", props.color.border.css_str().as_str()),
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
                        .css(Some(HashMap::from([
                            ("background-color", props.color.body_background.css_str().as_str()),
                        ])))
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
                                ("color", props.color.font.css_str().as_str()),
                                ("background-color", props.color.footer_background.css_str().as_str()),
                                ("border-bottom-color", props.color.border.css_str().as_str()),
                            ])))
                    }
                >
                    {footer_content}
                </div>
            }
        </div>
    }
}
