use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Post {}

pub enum Msg {}

impl Component for Post {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {"Post detail Page"}
            </div>
        }
    }
}
