use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use gloo_console::log;
use yew_agent::{Agent, AgentLink, Context, HandlerId};
pub use yew_agent::{Bridge, Bridged, Dispatched, Dispatcher};
pub use reqwasm::http::{Request, FormData};
use std::sync::{Mutex};
use std::rc::Rc;
use httpmessenger::{HttpMessengerAgent, Inputoutput};
use parking_lot::ReentrantMutex;
// use web_sys::FormData;
// use reqwasm::multipart::Multipart;

#[cfg(test)]
pub mod tests;

pub enum HttpAgentInput {
    Post {
        url: String,
        data: String,
        call_name: String,
        loader: bool
    },
    PostFormObj {
        url: String,
        data: FormData,
        call_name: String,
        loader: bool
    },
    Get {
        url: String,
        call_name: String,
        loader: bool
    },
    Delete {
        url: String,
        call_name: String,
        loader: bool
    }
}

impl HttpAgentInput {
    pub fn build_get(call_name: String, url: String, loader: bool) -> HttpAgentInput {
        HttpAgentInput::Get{url, call_name, loader}
    }

    pub fn build_post(call_name: String, url: String, data: String, loader: bool) -> HttpAgentInput {
        HttpAgentInput::Post {url, call_name, data, loader}
    }

    pub fn build_form(call_name: String, url: String, data: FormData, loader: bool) -> HttpAgentInput {
        HttpAgentInput::PostFormObj {url, call_name, data, loader}
    }

    pub fn build_delete(call_name: String, url: String, loader: bool) -> HttpAgentInput {
        HttpAgentInput::Delete {url, call_name, loader}
    }
}

pub enum HttpMessage {
    MessengerAgent(Inputoutput),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HttpAgentOutput {
    pub value: Option<String>,
    pub call_name: String,
    pub status_code: u16,
}

pub struct HttpAgent {
    link: Rc<ReentrantMutex<AgentLink<Self>>>,
    subscribers: HashSet<HandlerId>,
    messenger:  Rc<Mutex<Box<dyn Bridge<HttpMessengerAgent>>>>,
}

impl Agent for HttpAgent {
    type Reach = Context<Self>;
    type Message = HttpMessage;
    type Input = HttpAgentInput;
    type Output = HttpAgentOutput;

    fn create(link: AgentLink<Self>) -> Self {
        let messenger = HttpMessengerAgent::bridge(link.callback(Self::Message::MessengerAgent));

        Self {
            link: Rc::new(ReentrantMutex::new(link)),
            subscribers: HashSet::new(),
            messenger: Rc::new(Mutex::new(messenger))
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {

        let link = Rc::clone(&self.link);
        let linker_exec = &mut self.messenger.lock().unwrap();
        let messenger_link = Rc::clone(&self.messenger);

        match msg {
            Self::Input::Post{url, call_name, data, loader} => {

                // Check if loader is need than only activate the loader
                if loader {
                    linker_exec.send(Inputoutput::EnableLoader);
                }

                wasm_bindgen_futures::spawn_local(async move {
                    let result = Request::post(&url)
                                .header("Content-Type", "application/json")
                                .body(data).send().await;

                    let linker = link.lock();

                    let output = match result {
                        Ok(res) if res.status() == 200 => {

                            let data = res.text().await;
                            let text: Option<String> = data.ok();

                            Self::Output {
                                status_code: res.status(),
                                value: text,
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

                            let data = res.text().await;
                            let text: Option<String> = data.ok();

                            Self::Output {
                                status_code: res.status(),
                                value: text,
                                call_name: call_name
                            }
                        },
                    };
                    linker.respond(id, output);
                    if loader {
                        let linker_exec = &mut messenger_link.lock().unwrap();
                        linker_exec.send(Inputoutput::DisableLoader);
                    }
                });
            },

            Self::Input::PostFormObj{url, call_name, data, loader} => {

                // Check if loader is need than only activate the loader
                if loader {
                    linker_exec.send(Inputoutput::EnableLoader);
                }

                wasm_bindgen_futures::spawn_local(async move {
                    // let multipart = Multipart::new(&data);
                    let result = Request::post(&url)
                                .body(&data)
                                .send().await;

                    let linker = link.lock();

                    let output = match result {
                        Ok(res) if res.status() == 200 => {

                            let data = res.text().await;
                            let text: Option<String> = data.ok();

                            Self::Output {
                                status_code: res.status(),
                                value: text,
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

                            let data = res.text().await;
                            let text: Option<String> = data.ok();

                            Self::Output {
                                status_code: res.status(),
                                value: text,
                                call_name: call_name
                            }
                        },
                    };
                    linker.respond(id, output);
                    if loader {
                        let linker_exec = &mut messenger_link.lock().unwrap();
                        linker_exec.send(Inputoutput::DisableLoader);
                    }
                });
            },

            Self::Input::Get{url, call_name, loader} => {

                // Check if loader is need than only activate the loader
                // Same below 3 lines in Post also because we need loader data to togle
                // But loader data is not available before match statement
                if loader {
                    linker_exec.send(Inputoutput::EnableLoader);
                }

                wasm_bindgen_futures::spawn_local(async move {
                    let result = Request::get(&url)
                                .send().await;
                    let linker = link.lock();

                    let output = match result {
                        Ok(res) if res.status() == 200 => {

                            let data = res.text().await;
                            let text: Option<String> = data.ok();

                            Self::Output {
                                status_code: res.status(),
                                value: text,
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

                            let data = res.text().await;
                            let text: Option<String> = data.ok();

                            Self::Output {
                                status_code: res.status(),
                                value: text,
                                call_name: call_name
                            }
                        },
                    };
                    linker.respond(id, output);
                    if loader {

                        let linker_exec = &mut messenger_link.lock().unwrap();
                        linker_exec.send(Inputoutput::DisableLoader);

                    }
                });
            },
            Self::Input::Delete{url, call_name, loader} => {

                if loader {
                    linker_exec.send(Inputoutput::EnableLoader);
                }

                wasm_bindgen_futures::spawn_local(async move {
                    let result = Request::delete(&url)
                                .send().await;
                    let linker = link.lock();

                    let output = match result {
                        Ok(res) if res.status() == 200 => {

                            let data = res.text().await;
                            let text: Option<String> = data.ok();

                            Self::Output {
                                status_code: res.status(),
                                value: text,
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

                            let data = res.text().await;
                            let text: Option<String> = data.ok();

                            Self::Output {
                                status_code: res.status(),
                                value: text,
                                call_name: call_name
                            }
                        },
                    };
                    linker.respond(id, output);
                    if loader {

                        let linker_exec = &mut messenger_link.lock().unwrap();
                        linker_exec.send(Inputoutput::DisableLoader);

                    }
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
