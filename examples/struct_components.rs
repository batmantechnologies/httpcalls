use yew::prelude::*;
use httpmessenger::{StoreProvider, AppAction, StoreDispatcher, StoreContext};
use httpcalls::{HttpClient, HttpError, HttpResponse};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use gloo_console::log;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
    age: u32,
}

// Example 1: Basic GET request with struct component
pub struct UserListComponent {
    users: Vec<User>,
    error: Option<String>,
    dispatch: Option<StoreDispatcher>,
}

pub enum UserListMessage {
    FetchUsers,
    UsersLoaded(Vec<User>),
    Error(String),
    StoreContextChanged(StoreContext),
}

impl Component for UserListComponent {
    type Message = UserListMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            users: Vec::new(),
            error: None,
            dispatch: None,
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
                            .call_name("fetch_users")
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
                }
                false
            }
            UserListMessage::UsersLoaded(users) => {
                self.users = users;
                self.error = None;
                true
            }
            UserListMessage::Error(error) => {
                self.error = Some(error);
                true
            }
            UserListMessage::StoreContextChanged(context) => {
                self.dispatch = Some(context.dispatch);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch_users = ctx.link().callback(|_| UserListMessage::FetchUsers);

        html! {
            <ContextConsumer<StoreContext> context_callback={ctx.link().callback(UserListMessage::StoreContextChanged)}>
                <div class="user-list">
                    <h3>{"User List (Struct Component)"}</h3>
                    <button onclick={fetch_users}>{"Fetch Users"}</button>
                    
                    {if let Some(ref error) = self.error {
                        html! { <div class="error">{format!("Error: {}", error)}</div> }
                    } else {
                        html! {}
                    }}
                    
                    <div class="users">
                        {for self.users.iter().map(|user| {
                            html! {
                                <div class="user-card" key={user.id.unwrap_or(0)}>
                                    <h4>{&user.name}</h4>
                                    <p>{format!("Email: {}", &user.email)}</p>
                                    <p>{format!("Age: {}", user.age)}</p>
                                </div>
                            }
                        })}
                    </div>
                </div>
            </ContextConsumer<StoreContext>>
        }
    }
}

// Example 2: POST request with JSON payload
pub struct CreateUserComponent {
    result: Option<String>,
    error: Option<String>,
    dispatch: Option<StoreDispatcher>,
}

pub enum CreateUserMessage {
    CreateUser,
    UserCreated(String),
    Error(String),
    StoreContextChanged(StoreContext),
}

impl Component for CreateUserComponent {
    type Message = CreateUserMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            result: None,
            error: None,
            dispatch: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CreateUserMessage::CreateUser => {
                if let Some(ref dispatch) = self.dispatch {
                    let link = ctx.link().clone();
                    let http_client = HttpClient::with_dispatcher(dispatch.clone());
                    
                    spawn_local(async move {
                        let new_user = User {
                            id: None,
                            name: "John Doe".to_string(),
                            email: "john@example.com".to_string(),
                            age: 30,
                        };
                        
                        match http_client
                            .post("https://jsonplaceholder.typicode.com/users")
                            .json(&new_user)
                            .unwrap()
                            .with_loader(true)
                            .with_notifications(true)
                            .call_name("create_user")
                            .send()
                            .await
                        {
                            Ok(response) => {
                                let message = format!("User created! Status: {}", response.status);
                                link.send_message(CreateUserMessage::UserCreated(message));
                            }
                            Err(e) => {
                                link.send_message(CreateUserMessage::Error(format!("Failed to create user: {}", e)));
                            }
                        }
                    });
                }
                false
            }
            CreateUserMessage::UserCreated(result) => {
                self.result = Some(result);
                self.error = None;
                true
            }
            CreateUserMessage::Error(error) => {
                self.error = Some(error);
                self.result = None;
                true
            }
            CreateUserMessage::StoreContextChanged(context) => {
                self.dispatch = Some(context.dispatch);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let create_user = ctx.link().callback(|_| CreateUserMessage::CreateUser);

        html! {
            <ContextConsumer<StoreContext> context_callback={ctx.link().callback(CreateUserMessage::StoreContextChanged)}>
                <div class="create-user">
                    <h3>{"Create User (Struct Component)"}</h3>
                    <button onclick={create_user}>{"Create User"}</button>
                    
                    {if let Some(ref success) = self.result {
                        html! { <div class="success">{success}</div> }
                    } else {
                        html! {}
                    }}
                    
                    {if let Some(ref error) = self.error {
                        html! { <div class="error">{format!("Error: {}", error)}</div> }
                    } else {
                        html! {}
                    }}
                </div>
            </ContextConsumer<StoreContext>>
        }
    }
}

// Example 3: File upload with progress tracking
pub struct FileUploadComponent {
    upload_status: Option<String>,
    error: Option<String>,
    dispatch: Option<StoreDispatcher>,
}

pub enum FileUploadMessage {
    UploadFile,
    UploadComplete(String),
    Error(String),
    StoreContextChanged(StoreContext),
}

impl Component for FileUploadComponent {
    type Message = FileUploadMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            upload_status: None,
            error: None,
            dispatch: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FileUploadMessage::UploadFile => {
                if let Some(ref dispatch) = self.dispatch {
                    let link = ctx.link().clone();
                    let http_client = HttpClient::with_dispatcher(dispatch.clone());
                    
                    spawn_local(async move {
                        // Create some dummy file data
                        let file_data = b"Hello, this is a test file content!";
                        
                        let form_data = web_sys::FormData::new().unwrap();
                        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
                            &js_sys::Array::of1(&js_sys::Uint8Array::from(file_data.as_ref())),
                            web_sys::BlobPropertyBag::new(),
                        ).unwrap();
                        
                        form_data.append_with_blob_and_filename("file", &blob, "test.txt").unwrap();
                        
                        match http_client
                            .post("https://httpbin.org/post")
                            .form_data(form_data)
                            .with_loader(true)
                            .with_progress(true)
                            .call_name("upload_file")
                            .timeout(60000)
                            .send()
                            .await
                        {
                            Ok(response) => {
                                let message = format!("Upload complete! Status: {}", response.status);
                                link.send_message(FileUploadMessage::UploadComplete(message));
                            }
                            Err(e) => {
                                link.send_message(FileUploadMessage::Error(format!("Upload failed: {}", e)));
                            }
                        }
                    });
                }
                false
            }
            FileUploadMessage::UploadComplete(status) => {
                self.upload_status = Some(status);
                self.error = None;
                true
            }
            FileUploadMessage::Error(error) => {
                self.error = Some(error);
                self.upload_status = None;
                true
            }
            FileUploadMessage::StoreContextChanged(context) => {
                self.dispatch = Some(context.dispatch);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let upload_file = ctx.link().callback(|_| FileUploadMessage::UploadFile);

        html! {
            <ContextConsumer<StoreContext> context_callback={ctx.link().callback(FileUploadMessage::StoreContextChanged)}>
                <div class="file-upload">
                    <h3>{"File Upload (Struct Component)"}</h3>
                    <button onclick={upload_file}>{"Upload File"}</button>
                    
                    {if let Some(ref status) = self.upload_status {
                        html! { <div class="success">{status}</div> }
                    } else {
                        html! {}
                    }}
                    
                    {if let Some(ref error) = self.error {
                        html! { <div class="error">{format!("Error: {}", error)}</div> }
                    } else {
                        html! {}
                    }}
                </div>
            </ContextConsumer<StoreContext>>
        }
    }
}

// Example 4: Manual state management (without automatic integration)
pub struct ManualStateComponent {
    users: Vec<User>,
    is_loading: bool,
    error: Option<String>,
    dispatch: Option<StoreDispatcher>,
}

pub enum ManualStateMessage {
    FetchUsers,
    UsersLoaded(Vec<User>),
    Error(String),
    StoreContextChanged(StoreContext),
}

impl Component for ManualStateComponent {
    type Message = ManualStateMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            users: Vec::new(),
            is_loading: false,
            error: None,
            dispatch: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ManualStateMessage::FetchUsers => {
                self.is_loading = true;
                self.error = None;
                
                // Manual loader state management
                if let Some(ref dispatch) = self.dispatch {
                    dispatch.emit(AppAction::EnableLoader);
                }
                
                let link = ctx.link().clone();
                let dispatch = self.dispatch.clone();
                
                spawn_local(async move {
                    // Create HTTP client without automatic state integration
                    let http_client = HttpClient::new();
                    
                    match http_client
                        .get("https://jsonplaceholder.typicode.com/users")
                        .call_name("fetch_users_manual")
                        .send()
                        .await
                    {
                        Ok(response) => {
                            // Manual loader disable
                            if let Some(dispatch) = dispatch {
                                dispatch.emit(AppAction::DisableLoader);
                            }
                            
                            match response.json::<Vec<User>>() {
                                Ok(users) => {
                                    link.send_message(ManualStateMessage::UsersLoaded(users));
                                }
                                Err(e) => {
                                    link.send_message(ManualStateMessage::Error(format!("Parse error: {}", e)));
                                }
                            }
                        }
                        Err(e) => {
                            // Manual loader disable on error
                            if let Some(dispatch) = dispatch {
                                dispatch.emit(AppAction::DisableLoader);
                            }
                            link.send_message(ManualStateMessage::Error(format!("Request error: {}", e)));
                        }
                    }
                });
                
                true
            }
            ManualStateMessage::UsersLoaded(users) => {
                self.users = users;
                self.is_loading = false;
                self.error = None;
                true
            }
            ManualStateMessage::Error(error) => {
                self.error = Some(error);
                self.is_loading = false;
                true
            }
            ManualStateMessage::StoreContextChanged(context) => {
                self.dispatch = Some(context.dispatch);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch_users = ctx.link().callback(|_| ManualStateMessage::FetchUsers);

        html! {
            <ContextConsumer<StoreContext> context_callback={ctx.link().callback(ManualStateMessage::StoreContextChanged)}>
                <div class="manual-state">
                    <h3>{"Manual State Management (Struct Component)"}</h3>
                    <button onclick={fetch_users} disabled={self.is_loading}>
                        {if self.is_loading { "Loading..." } else { "Fetch Users" }}
                    </button>
                    
                    {if let Some(ref error) = self.error {
                        html! { <div class="error">{format!("Error: {}", error)}</div> }
                    } else {
                        html! {}
                    }}
                    
                    <div class="users">
                        {for self.users.iter().map(|user| {
                            html! {
                                <div class="user-card" key={user.id.unwrap_or(0)}>
                                    <h4>{&user.name}</h4>
                                    <p>{format!("Email: {}", &user.email)}</p>
                                </div>
                            }
                        })}
                    </div>
                </div>
            </ContextConsumer<StoreContext>>
        }
    }
}

// Main component showcasing all struct component examples
pub struct MainContent {
    _phantom: (),
}

impl Component for MainContent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { _phantom: () }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="struct-examples">
                <h1>{"Struct Component Examples"}</h1>
                <p>{"Using httpcalls and httpmessenger with traditional struct components"}</p>
                
                <div class="examples-grid">
                    <UserListComponent />
                    <CreateUserComponent />
                    <FileUploadComponent />
                    <ManualStateComponent />
                </div>
                
                <div class="usage-notes">
                    <h2>{"Usage Notes for Struct Components"}</h2>
                    <ul>
                        <li>{"Use ContextConsumer to access StoreContext"}</li>
                        <li>{"Store the dispatcher in component state"}</li>
                        <li>{"Create HttpClient with dispatcher for automatic integration"}</li>
                        <li>{"Use spawn_local for async HTTP requests"}</li>
                        <li>{"Send messages back to component via ctx.link()"}</li>
                        <li>{"Manual state management is also possible if preferred"}</li>
                    </ul>
                </div>
            </div>
        }
    }
}

pub struct App {
    _phantom: (),
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { _phantom: () }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <StoreProvider>
                <div class="app">
                    <httpmessenger::Loader message="Loading..." show_progress={true} />
                    <httpmessenger::NotificationManager auto_hide_duration={5000} />
                    <MainContent />
                </div>
            </StoreProvider>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}