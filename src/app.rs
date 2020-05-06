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
        let value = {
            if let Json(Ok(restored_entries)) = storage.restore(KEY) {
                restored_entries
            } else {
                "".to_string()
            }
        };
        let state = State {
            value,
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
                <header id="header">
                    {"Yew Markdown Memo"}
                </header>
                <article id="article">
                    <div class="l-column">
                        <h2 class="section-title">{"Markdown"}</h2>
                        <textarea
                          class="markdown"
                          oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                          value=self.state.value
                        />
                    </div>
                    <div class="l-column">
                        <h2 class="section-title">{"Preview"}</h2>
                        <div class="preview">{&self.state.value}</div>
                    </div>
                </article>
            </>
        }
    }
}
