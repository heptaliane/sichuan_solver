use wasm_logger;
use yew::prelude::*;

mod view_components;

enum AppMsg {
    TabChanged(view_components::main_container::MainContainerViewTarget),
}

struct App {
    target_view: view_components::main_container::MainContainerViewTarget,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            target_view: view_components::main_container::MainContainerViewTarget::EditView,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <view_components::main_container::MainContainer
                    target_view={self.target_view}
                    on_tab_change={
                        ctx.link().callback(|target| Self::Message::TabChanged(target))
                    }
                />
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
