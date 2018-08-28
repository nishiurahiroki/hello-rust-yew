use yew::prelude::{Component, Html, ComponentLink, ShouldRender, Renderable};

pub struct Header {
    text : String
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct Props {
    pub text : String
}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props : Self::Properties, _ : ComponentLink<Self>) -> Self {
        Header {
            text : props.text
        }
    }

    fn update(&mut self, _ : Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _ : Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Header> for Header {
    fn view(&self) ->Html<Self> {
        html! {
            <div>
                <h1>{&self.text}</h1>
            </div>
        }
    }
}
