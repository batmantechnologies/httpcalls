use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use gloo_console::log;
use yew_agent::{Agent, AgentLink, Context, HandlerId};
pub use yew_agent::{Bridge, Bridged, Dispatched, Dispatcher};
use reqwasm::http::{Request};
use std::sync::{Mutex};
use std::rc::Rc;
#[cfg(test)]
pub mod tests;

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

    pub fn build_post(call_name: String, url: String, data: String) -> HttpAgentInput {
        HttpAgentInput::Post {url, call_name, data}
    }
}

#[derive(Serialize, Deserialize)]
pub struct HttpAgentOutput {
    pub value: Option<String>,
    pub call_name: String,
    pub status_code: u16
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
                    let result = Request::post(&url)
                                .header("Content-Type", "application/json")
                                .body(data).send().await;

                    let linker = link.lock().unwrap();

                    let output = match result {
                        Ok(res) if res.status() == 200 => {

                            Self::Output {
                                status_code: res.status(),
                                value: Some(res.text().await.unwrap()),
                                call_name: call_name
                            }
                        },
                        Err(_) => {
                            Self::Output {
                                status_code: 404,
                                value: None,
                                call_name: call_name
                            }
                        },
                        Ok(res) => {
                            Self::Output {
                                status_code: res.status(),
                                value: Some(res.text().await.unwrap()), // res.body.u,
                                call_name: call_name
                            }
                        },
                    };
                    linker.respond(id, output);
                });
            },
            Self::Input::Get{url, call_name} => {
                wasm_bindgen_futures::spawn_local(async move {
                    let result = Request::get(&url)
                        .send()
                        .await;

                    let linker = link.lock().unwrap();

                    let output = match result {
                        Ok(res) if res.status() == 200 => {
                            Self::Output {
                                status_code: res.status(),
                                value: Some(res.text().await.unwrap()),
                                call_name: call_name
                            }
                        },
                        Err(_) => {
                            Self::Output {
                                status_code: 404,
                                value: None,
                                call_name: call_name
                            }
                        },
                        Ok(res) => {
                            Self::Output {
                                status_code: res.status(),
                                value: Some(res.text().await.unwrap()), 
                                call_name: call_name
                            }
                        },
                    };
                    linker.respond(id, output);
                });
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
