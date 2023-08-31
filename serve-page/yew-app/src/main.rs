use yew::prelude::*;
use wasm_bindgen::JsValue;
use log::info;

use yew::prelude::*;

struct Model {
    counter: i32,
}

enum Msg {
    Increment,
    Decrement,
}

#[derive(PartialEq, Properties, Default)]
pub struct Props;

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Model { counter: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message ) -> bool {
        match msg {
            Msg::Increment => {
                self.counter += 1;
            }
            Msg::Decrement => {
                self.counter -= 1;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ self.counter }</h1>
                <button onclick={ctx.link().callback(|_| Msg::Increment)}>{ "+1" }</button>
                <button onclick={ctx.link().callback(|_| Msg::Decrement)}>{ "-1" }</button>
            </div>
        }
    }
}
//

fn main() {
    yew::Renderer::<Model>::new().render();
}

