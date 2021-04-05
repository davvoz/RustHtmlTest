#![recursion_limit = "512"]
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    canvas_width: i64,
}

enum Msg {
    AddOne,
    SubOne,
    Draw,
    Draw2,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            canvas_width: 500,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.canvas_width = add(self.canvas_width, 10),
            Msg::SubOne => self.canvas_width = add(self.canvas_width, -10),
            Msg::Draw => draw_canvas(),
            Msg::Draw2 => draw_other_canvas(),
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
            <div style="margin-left: -0.50%;">
                <div style="background-color: black;
                            color:white;
                            top: 0;
                            position: fixed;
                            margin: 0;
                            width: 100%;
                            height: 10%;
                            text-overflow: ellipsis;">
                    <h1>{"Hello world ! HTML in Rust using Yew"}</h1>
                </div>
                <div style="background-color: green;
                            color: black;
                            top: 10%;
                            position: fixed;
                            margin: 0;
                            width: 100%;
                            height: 70%;
                            text-overflow: ellipsis;">
                    <div style="background-color:
                                pink;
                                width:33.4%;
                                float:left;
                                position:fixed;
                                height:60%">
                        <div>{"Canvas width"}</div>
                        <div>
                            <button onclick=self.link.callback(|_| Msg::Draw2)>{ "draw" }</button>
                        </div>
                        <canvas id="canvas" style="background-color:white;width:100%"/>
                    </div>
                    <div style="background-color:yellow;
                                width:33.33%;
                                float:left;
                                position:fixed;
                                left:33.33%;
                                height:60%">
                        {"lorem ips"}
                        <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                            <button onclick=self.link.callback(|_| Msg::SubOne)>{ "-1" }</button>
                        <p>{ self.canvas_width }</p>
                    </div>
                    <div style="background-color:grey;
                                width:33.33%;
                                float:left;
                                position:fixed;
                                left:66.66%;
                                height:60%">
                        {"lorem ips"}
                        <div>
                            <button onclick=self.link.callback(|_| Msg::Draw)>{ "draw" }</button>
                        </div>
                        <canvas id="canvasz"  style="background-color:white;width:100%"/>
                    </div>
                </div>
                <div style="background-color: blue;
                            color: white;
                            top: 80%;
                            position: fixed;
                            margin: 0;
                            width: 100%;
                            height: 20%;
                            text-overflow: ellipsis;">
                        <h3>{"Bye bye Cruel world !"}</h3>
                </div>
            </div>
        }
    }
}
pub fn initialize() {}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

fn draw_canvas() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvasz").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}
fn draw_other_canvas() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let height = canvas.height();
    let width = canvas.width();

    let lato = 20;
    let mut _counter_hight = 0;
    let mut _counter_width = 0;

    for y in 0..width * lato {
        for x in 0..height * lato {
            if y % 2 == 0 {
                context.set_fill_style(&"red".into());
            } else if y % 2 == 1 {
                context.set_fill_style(&"green".into());
            }
            context.fill_rect(
                (y * lato).into(),
                (x * lato).into(),
                lato.into(),
                lato.into(),
            );
            _counter_width += 1 * lato;
        }
        _counter_hight += 1 * lato;
    }
}
