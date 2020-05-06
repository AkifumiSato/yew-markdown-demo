use log::*;
use serde_derive::{Deserialize, Serialize};
use pulldown_cmark::{html::push_html, Options, Parser};
use stdweb::web::Node;
use yew::virtual_dom::VNode;
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
    text: String,
}

pub enum Msg {
    Update(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let text = {
            if let Json(Ok(restored_entries)) = storage.restore(KEY) {
                restored_entries
            } else {
                "".to_string()
            }
        };
        let state = State {
            text,
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
                self.state.text = val;
            }
        }
        self.storage.store(KEY, Json(&self.state.text));
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");

        let parse_html = parse_text(&self.state.text);
        let html_text = format!("<div>{}</div>", &parse_html);
        let node = Node::from_html(&html_text).unwrap();
        let preview = VNode::VRef(node);

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
                          value=self.state.text
                        />
                    </div>
                    <div class="l-column">
                        <h2 class="section-title">{"Preview"}</h2>
                        <div class="preview">{preview}</div>
                    </div>
                </article>
            </>
        }
    }
}

fn parse_text(value: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&value, options);
    let mut parsed_text = String::new();
    push_html(&mut parsed_text, parser);

    parsed_text
}