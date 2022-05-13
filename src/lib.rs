use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use gloo_console::log;
use yew_agent::{Agent, AgentLink, Context, HandlerId};
pub use yew_agent::{Bridge, Bridged, Dispatched, Dispatcher};
use reqwasm::http::{Request};
use std::sync::{Mutex};
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
pub enum HttpAgentInput {
    Post {
        url: String,
        data: String,
        call_name: String,
    },
    Get {
        url: String,
        call_name: String,
    }
}

impl HttpAgentInput {
    pub fn build_get(url: String, call_name: String) -> HttpAgentInput {
        HttpAgentInput::Get{url, call_name}
    }

    pub fn build_post(url: String, call_name: String, data: String) -> HttpAgentInput {
        HttpAgentInput::Post {url, call_name, data}
    }
}

#[derive(Serialize, Deserialize)]
pub struct HttpAgentOutput {
    pub value: Option<String>,
    pub call_name: String,
}

pub struct HttpAgent {
    link: Rc<Mutex<AgentLink<Self>>>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for HttpAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = HttpAgentInput;
    type Output = HttpAgentOutput;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link: Rc::new(Mutex::new(link)),
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {

        let link = Rc::clone(&self.link);

        match msg {
            Self::Input::Post{url, call_name, data} => {
                wasm_bindgen_futures::spawn_local(async move {
                    let result = Request::post(&url).header("Content-Type", "application/json")
                        .body(data).send().await;

                    match result {
                        Ok(res) if res.status() == 200 => {
                            let output = Self::Output {
                                value: Some("Hello ".to_string()),
                                call_name: call_name
                            };
                            let linker = link.lock().unwrap();
                            linker.respond(id, output);
                        },
                        Err(_) => {
                            log!("404");
                        },
                        Ok(res) => {
                            log!(res.status().clone());
                        },
                    };
                });
            },
            Self::Input::Get{url, call_name} => {
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
