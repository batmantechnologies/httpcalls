use yew::prelude::*;
use httpmessenger::{StoreProvider, StoreDispatcher, StoreContext};
use httpcalls::HttpClient;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
    // Removed age to match jsonplaceholder
}

// Example 1: Basic GET request with struct component
pub struct UserListComponent {
    users: Vec<User>,
    error: Option<String>,
    dispatch: Option<StoreDispatcher>,
    _context_listener: ContextHandle<Rc<StoreContext>>,
}

pub enum UserListMessage {
    FetchUsers,
    UsersLoaded(Vec<User>),
    Error(String),
    StoreContextChanged(Rc<StoreContext>),
}

impl Component for UserListComponent {
    type Message = UserListMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (context_rc, context_listener) = ctx
            .link()
            .context(ctx.link().callback(UserListMessage::StoreContextChanged))
            .expect("No StoreContext provider found. Make sure <StoreProvider> is an ancestor.");

        let dispatch = Some(context_rc.dispatch.clone());

        // Automatically fetch users if context is available
        if dispatch.is_some() {
            ctx.link().send_future(async {
                UserListMessage::FetchUsers
            });
        }

        Self {
            users: Vec::new(),
            error: None,
            dispatch,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserListMessage::FetchUsers => {
                if let Some(ref dispatch) = self.dispatch {
                    let link = ctx.link().clone();
                    let http_client = HttpClient::with_dispatcher(dispatch.clone());
                    
                    spawn_local(async move {
                        match http_client
                            .get("https://jsonplaceholder.typicode.com/users")
                            .with_loader(true)
                            .call_name("fetch_users_struct")
                            .send()
                            .await
                        {
                            Ok(response) => {
                                match response.json::<Vec<User>>() {
                                    Ok(users) => {
                                        link.send_message(UserListMessage::UsersLoaded(users));
                                    }
                                    Err(e) => {
                                        link.send_message(UserListMessage::Error(format!("Parse error: {}", e)));
                                    }
                                }
                            }
                            Err(e) => {
                                link.send_message(UserListMessage::Error(format!("Request error: {}", e)));
                            }
                        }
                    });
                } else {
                    self.error = Some("Context not available yet for dispatching FetchUsers".to_string());
                    return true;
                }
                false // No re-render needed immediately, wait for UsersLoaded or Error
            }
            UserListMessage::UsersLoaded(users) => {
                self.users = users;
                self.error = None;
                true
            }
            UserListMessage::Error(error_msg) => {
                self.error = Some(error_msg);
                true
            }
            UserListMessage::StoreContextChanged(context_rc) => {
                let had_dispatch_before = self.dispatch.is_some();
                self.dispatch = Some(context_rc.dispatch.clone());
                // If dispatch wasn't available before, and now it is, try fetching.
                if !had_dispatch_before && self.users.is_empty() && self.error.is_none() {
                    ctx.link().send_message(UserListMessage::FetchUsers);
                }
                true // Re-render as dispatch availability might affect button state
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch_users = ctx.link().callback(|_| UserListMessage::FetchUsers);

        html! {
            <div class="user-list-struct-example">
                <h3>{"User List (Struct Component)"}</h3>
                <button onclick={fetch_users} disabled={self.dispatch.is_none()}>{"Fetch Users"}</button>
                
                {if let Some(ref error_message) = self.error {
                    html! { <p class="error">{ error_message }</p> }
                } else {
                    html! {}
                }}
                
                {if self.users.is_empty() && self.error.is_none() && self.dispatch.is_some() {
                    html! { <p>{ "Click 'Fetch Users' to load data." }</p> }
                } else if self.users.is_empty() && self.error.is_none() && self.dispatch.is_none() {
                     html!{ <p>{ "Waiting for context..."}</p>}
                } else {
                    html!{}
                }}

                <ul>
                    {for self.users.iter().map(|user| {
                        html! {
                            <li key={user.id.unwrap_or_default()}>
                                {&user.name}{" ("}{&user.email}{")"}
                            </li>
                        }
                    })}
                </ul>
            </div>
        }
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <StoreProvider>
                <httpmessenger::Loader message="Loading from struct example..." show_progress={true} />
                <httpmessenger::NotificationManager auto_hide_duration={5000} />
                <h1>{"Struct Component HTTPCalls Example"}</h1>
                <UserListComponent />
            </StoreProvider>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}