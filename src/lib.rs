use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div style="background-color: black;
                            color: grey;
                            top: 0;
                            position: fixed;
                            margin: 0;
                            width: 100%;
                            height: 10%;
                            text-overflow: ellipsis;">
                    <h1>{"Hello world ! I'm trying to control HTML from Rust"}</h1>
                </div>
                <div style="background-color: green;
                            color: black;
                            top: 10%;
                            position: fixed;
                            margin: 0;
                            width: 100%;
                            height: 70%;
                            text-overflow: ellipsis;">
                    <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                    <p>{ self.value }</p>
                </div>
                <div style="background-color: blue;
                            color: black;
                            top: 80%;
                            position: fixed;
                            margin: 0;
                            width: 100%;
                            height: 20%;
                            text-overflow: ellipsis;">
                            <p>{"Bye bye Cruel world !"}</p>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
