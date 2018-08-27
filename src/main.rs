#[macro_use]
extern crate yew;
use yew::prelude::*;

struct Model {
    texts : Vec<String>
}

enum Msg {
    AddText
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ : Self::Properties, _ : ComponentLink<Self>) -> Self {
        Model {
            texts : Vec::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddText => {
                self.texts.push("added text".to_string());
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let render = |text : &String| html! {
            <div>{text}</div>
        };

        html! {
            <div>
                {"hello! yew!"}
                <button onclick=|_| Msg::AddText,>{"Add Text!"}</button>
                {for self.texts.iter().map(render)}
            </div>
        }
    }
}


fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
