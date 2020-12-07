use yew::prelude::*;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        let text = "text";

        html! {
            <>
                <h1>{"Heading"}</h1>
                <p>{"This is the paragraph."}</p>
                <p>{text}</p>
                <a href="https://yew.rs/docs/zh-CN">{"Yew Docs"}</a>
                <div>
                    <button> { "Click me!" } </button>
                </div>
            </>
        }
    }
}
