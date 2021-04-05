#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;
//use gloo::{events::EventListener};

struct Model {
    link: ComponentLink<Self>,
    stroke_style_is_red: bool,
    stroke_style_is_red_c2: bool,
}

enum Msg {
    Draw2,
    Try,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            stroke_style_is_red_c2: true,
            stroke_style_is_red: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if self.stroke_style_is_red == true {
            self.stroke_style_is_red = false;
        } else {
            self.stroke_style_is_red = true;
        }
        if self.stroke_style_is_red_c2 == true {
            self.stroke_style_is_red_c2 = false;
        } else {
            self.stroke_style_is_red_c2 = true;
        }
        match msg {
            Msg::Draw2 => try_it(self.stroke_style_is_red_c2, 0),
            Msg::Try => try_it(self.stroke_style_is_red, 1),
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
                                width:50%;
                                float:left;
                                position:fixed;
                                height:60%">
                        <canvas id="canvas" style="background-color:white;width:100%" onclick=self.link.callback(|_| Msg::Draw2)/>
                    </div>
                    <div style="background-color:grey;
                                width:50%;
                                float:left;
                                position:fixed;
                                left:50%;
                                height:60%">
                        <canvas id="canvasz"  style="background-color:white;width:100%" onclick=self.link.callback(|_| Msg::Try)/>
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

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

fn try_it(is_red: bool, canvas_index: i8) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas;
    if canvas_index == 1 {
        canvas = document.get_element_by_id("canvasz").unwrap();
    } else {
        canvas = document.get_element_by_id("canvas").unwrap();
    }

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

    let lato = canvas.width() / 10;
    let height = canvas.height();
    let width = canvas.width();
    context.set_line_width(10.0);
    if is_red == true {
        context.set_stroke_style(&"green".into());
    } else {
        context.set_stroke_style(&"red".into());
    }

    for y in 0..height {
        for x in 0..width {
            context.stroke_rect(
                (y * lato).into(),
                (x * lato).into(),
                lato.into(),
                lato.into(),
            );
        }
    }
}

pub fn eventlistener_new_p_mousedown() {
    // let document = web_sys::window().unwrap().document().unwrap();
    // let canvas = document.get_element_by_id("canvas").unwrap();
    // let canvas: web_sys::HtmlCanvasElement = canvas
    //     .dyn_into::<web_sys::HtmlCanvasElement>()
    //     .map_err(|_| ())
    //     .unwrap();
    // let on_down = EventListener::new(&canvas, "mousedown", move |_event| {
    //     web_sys::console::log_1(&"Paragrapah mousedown".into());
    //     try_it(false);
    // });
    // on_down.forget();
}
