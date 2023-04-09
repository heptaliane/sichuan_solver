use yew::callback::Callback;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::Properties;

use super::super::components::Tile;
use super::card::Card;
use super::edit_view_control_tile_selector::EditViewControlTileSelector;
use super::input_form_control_element::InputFormControlElement;

#[derive(Properties, PartialEq)]
pub struct EditViewControlProps {
    pub rows: usize,
    pub cols: usize,
    pub tile: Option<Tile>,

    pub on_rows_change: Callback<usize>,
    pub on_cols_change: Callback<usize>,
    pub on_tile_change: Callback<Option<Tile>>,
}

#[function_component(EditViewControl)]
pub fn edit_view_control(props: &EditViewControlProps) -> Html {
    let on_rows_change = props.on_rows_change.clone();
    let on_cols_change = props.on_cols_change.clone();
    let on_tile_change = props.on_tile_change.clone();

    html! {
        <div>
            <Card header="Matrix size">
                <div class="row">
                    <div class="col-6">
                        <InputFormControlElement
                            label="Rows"
                            input_type="number"
                            value={AttrValue::from(props.rows.to_string())}
                            onchange={Callback::from(move |v: AttrValue| {
                                if let Ok(value) = v.to_string().parse::<usize>() {
                                    on_rows_change.emit(value);
                                }
                            })}
                        />
                    </div>
                    <div class="col-6">
                        <InputFormControlElement
                            label="Cols"
                            input_type="number"
                            value={AttrValue::from(props.cols.to_string())}
                            onchange={Callback::from(move |v: AttrValue| {
                                if let Ok(value) = v.to_string().parse::<usize>() {
                                    on_cols_change.emit(value);
                                }
                            })}
                        />
                    </div>
                </div>
            </Card>
            <Card header="Tile selector">
                <EditViewControlTileSelector
                    selected={props.tile}
                    onchange={Callback::from(move |tile: Option<Tile>| {
                        on_tile_change.emit(tile);
                    })}
                />
            </Card>
        </div>
    }
}
