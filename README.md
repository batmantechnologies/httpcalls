# httpcalls
This is Asynchronous Rust-Wasm interface. this is just Ajax kind of library to wasm "Yew" projects.


It became very difficult to find an working model of yew-agent for async http calls.
Being from javascript/jquery/angular/Vue  I had difficulti making things work,
Has spend many hours searching and correcting it,

This httpcalls is opensource now, it is yew agent for http calls. this
behaves like ajax in jquery, this triggers update method of Component.

I couln't get tests up and running hence pasted mu production usage code.

We are developping a huge huge wasm ecommerce web applicaiton, Entire stack based on just
Rust and Rust frameworks both back end and front end,


```
use yew::prelude::*;
use std::ops::Deref;
use web_sys::{HtmlInputElement, HtmlElement,
              Element,
              EventTarget,
              Node,
              NamedNodeMap, DomTokenList};
use gloo_console::log;
use httpcalls::{HttpAgent, HttpAgentOutput, HttpWorkerInput};
use httpcalls::{Bridge, Bridged};
use std::collections::HashMap;
// use wasm_bindgen::{prelude::Cuse wasm_bindgen::JsCast;losure, JsCast};
use wasm_bindgen::JsCast;


pub struct UserRegisterPage {
    username: String,
    app_id: i32,
    httpAgent: Box<dyn Bridge<HttpAgent>>,
    node_refs: HashMap<String, NodeRef>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub app_id: i32
}

pub enum Message {
    SetUsername(String),
    Register,
    GotHttpAgentOutput(HttpAgentOutput),
    None
}

impl Component for UserRegisterPage {

    type Message = Message;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {

        // When api call is over an update method is triggered with msg `Self::Message::GotHttpAgentOutput`
        let worker = HttpAgent::bridge(ctx.link().callback(Self::Message::GotHttpAgentOutput));
        let props = ctx.props();

        let mut refs: HashMap::<String, NodeRef> =  HashMap::new();
        refs.insert("html_input".into(), NodeRef::default());

        UserRegisterPage {
            app_id: props.app_id,
            httpAgent: worker,
            username:"".into(),
            node_refs: refs
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            Self::Message::Register => {
                let data =  json!({"url": "/testing-force/".to_owned()}).to_string();
                self.httpworker.send(HttpWorkerInput::build_post(
                    "/master-permission/apis/add/".into(),
                    "tester".to_string(),
                    data,
                ));
            },
            Self::Message::GotHttpAgentOutput(data) => {
                log!("{}", data.call_name);
            },
            Self::Message::SetUsername(username) => {
                self.username = username
            },
            Self::Message::None => {

            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let link = ctx.link();

        let set_username = link.callback(|e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            Self::Message::SetUsername(input.unwrap().value())
        });

        let onsubmit = link.callback(|e: FocusEvent| {
            e.prevent_default();
            e.stop_propagation();
            let form = e.target_dyn_into::<HtmlElement>().unwrap();
            let username = form.get_elements_by_tag_name("input").named_item("username").unwrap();
            let input = username.dyn_into::<HtmlInputElement>().unwrap();
            if input.value().chars().count() == 0_usize {
                input.class_list().add_1("border-danger");
                Self::Message::None
            } else {
                input.class_list().remove_1("border-danger");
                Self::Message::Register    
            }
        });


        html! {
            <>
                //    <!-- START SECTION BREADCRUMB -->
                <div class="login_register_wrap section">
                    <div class="container">
                        <div class="row d-flex justify-content-center">
                              <div class="col-md-6 col-md-6">
                                    <div class="login_wrap">
                                		<div class="padding_eight_all bg-white">
                                            <div class="heading_s1">
                                                <h3>{"Welcome to Bluebasket"}</h3>
                                            </div>
                                            <form {onsubmit}>
                                                <div class="form-group mb-3">
                                                    <input name="username" ref={self.node_refs.get("html_input").unwrap().clone()} onchange={set_username} type="text" class="form-control" placeholder="Your Email" />
                                                    <div class="invalid-feedback">
                                                        {"Please choose a username."}
                                                    </div>
                                                </div>

                                                <div class="form">
                                                    <button class="btn btn-fill-out btn-block" type="submit" role="button">{"Register"}</button>
                                                </div>
                                                <p>{"You will receive OTP to your email"}</p>
                                            </form>
                                        </div>
                                     </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </>
            }
        }
    }

```
