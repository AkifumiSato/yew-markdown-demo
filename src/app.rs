use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "yew.markdown.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    value: String,
}

pub enum Msg {
    Update(String),
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let state = State {
            value: "".to_string(),
        };
        App {
            link,
            storage,
            state,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::Nope => {}
        }
        self.storage.store(KEY, Json(&self.state.value));
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <>
                <header>
                    <p>
                        {"Yew Markdown Preview: "}
                    </p>
                </header>
                <div class={"container"}>
                    <textarea oninput=self.link.callback(|e: InputData| Msg::Update(e.value)) />
                    <div>{&self.state.value}</div>
                </div>
            </>
        }
    }
}
