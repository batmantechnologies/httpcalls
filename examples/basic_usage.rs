use yew::prelude::*;
use httpmessenger::{StoreProvider, AppAction};
use httpcalls::{HttpClient, HttpError, use_http_client};
use serde::{Deserialize, Serialize};
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
    age: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ApiResponse<T> {
    data: T,
    message: String,
    success: bool,
}

/// Example of basic GET request
#[function_component(GetExample)]
pub fn get_example() -> Html {
    let users = use_state(|| Vec::<User>::new());
    let error = use_state(|| None::<String>);
    let http_client = use_http_client();

    let fetch_users = {
        let users = users.clone();
        let error = error.clone();
        let http_client = http_client.clone();
        
        Callback::from(move |_| {
            let users = users.clone();
            let error = error.clone();
            let http_client = http_client.clone();
            
            spawn_local(async move {
                error.set(None);
                
                match http_client
                    .get("https://jsonplaceholder.typicode.com/users")
                    .with_loader(true)
                    .call_name("fetch_users")
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.json::<Vec<User>>() {
                            Ok(user_list) => {
                                log!("Fetched {} users", user_list.len());
                                users.set(user_list);
                            }
                            Err(e) => {
                                error.set(Some(format!("Parse error: {}", e)));
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Request error: {}", e)));
                    }
                }
            });
        })
    };

    html! {
        <div class="get-example">
            <h3>{"GET Request Example"}</h3>
            <button onclick={fetch_users}>{"Fetch Users"}</button>
            
            {if let Some(ref err) = *error {
                html! { <div class="error">{format!("Error: {}", err)}</div> }
            } else {
                html! {}
            }}
            
            <div class="users-list">
                {for users.iter().map(|user| {
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
    }
}

/// Example of POST request with JSON payload
#[function_component(PostExample)]
pub fn post_example() -> Html {
    let result = use_state(|| None::<String>);
    let error = use_state(|| None::<String>);
    let http_client = use_http_client();

    let create_user = {
        let result = result.clone();
        let error = error.clone();
        let http_client = http_client.clone();
        
        Callback::from(move |_| {
            let result = result.clone();
            let error = error.clone();
            let http_client = http_client.clone();
            
            spawn_local(async move {
                result.set(None);
                error.set(None);
                
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
                        log!("User created successfully: {}", response.status);
                        result.set(Some(format!("User created! Status: {}", response.status)));
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to create user: {}", e)));
                    }
                }
            });
        })
    };

    html! {
        <div class="post-example">
            <h3>{"POST Request Example"}</h3>
            <button onclick={create_user}>{"Create User"}</button>
            
            {if let Some(ref success) = *result {
                html! { <div class="success">{success}</div> }
            } else {
                html! {}
            }}
            
            {if let Some(ref err) = *error {
                html! { <div class="error">{format!("Error: {}", err)}</div> }
            } else {
                html! {}
            }}
        </div>
    }
}

/// Example of file upload with progress tracking
#[function_component(UploadExample)]
pub fn upload_example() -> Html {
    let upload_status = use_state(|| None::<String>);
    let error = use_state(|| None::<String>);
    let http_client = use_http_client();

    let upload_file = {
        let upload_status = upload_status.clone();
        let error = error.clone();
        let http_client = http_client.clone();
        
        Callback::from(move |_| {
            let upload_status = upload_status.clone();
            let error = error.clone();
            let http_client = http_client.clone();
            
            spawn_local(async move {
                upload_status.set(None);
                error.set(None);
                
                // Create some dummy file data
                let file_data = b"Hello, this is a test file content!";
                
                let form_data = web_sys::FormData::new().unwrap();
                let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
                    &js_sys::Array::of1(&js_sys::Uint8Array::from(file_data.as_ref())),
                    web_sys::BlobPropertyBag::new().type_("text/plain"),
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
                        log!("File uploaded successfully: {}", response.status);
                        upload_status.set(Some(format!("Upload complete! Status: {}", response.status)));
                    }
                    Err(e) => {
                        error.set(Some(format!("Upload failed: {}", e)));
                    }
                }
            });
        })
    };

    html! {
        <div class="upload-example">
            <h3>{"File Upload Example"}</h3>
            <button onclick={upload_file}>{"Upload File"}</button>
            
            {if let Some(ref status) = *upload_status {
                html! { <div class="success">{status}</div> }
            } else {
                html! {}
            }}
            
            {if let Some(ref err) = *error {
                html! { <div class="error">{format!("Error: {}", err)}</div> }
            } else {
                html! {}
            }}
        </div>
    }
}

/// Example with custom headers and authentication
#[function_component(AuthExample)]
pub fn auth_example() -> Html {
    let result = use_state(|| None::<String>);
    let error = use_state(|| None::<String>);

    let fetch_protected_data = {
        let result = result.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let result = result.clone();
            let error = error.clone();
            
            spawn_local(async move {
                result.set(None);
                error.set(None);
                
                // Create client with base URL and default headers
                let http_client = HttpClient::new()
                    .base_url("https://api.example.com")
                    .default_header("Authorization", "Bearer your-token-here")
                    .default_header("X-API-Version", "v1");
                
                match http_client
                    .get("/protected/data")
                    .header("Accept", "application/json")
                    .with_loader(true)
                    .call_name("fetch_protected")
                    .retry(3, 1000) // Retry 3 times with 1 second delay
                    .send()
                    .await
                {
                    Ok(response) => {
                        log!("Protected data fetched: {}", response.status);
                        result.set(Some(format!("Success! Response: {}", response.text())));
                    }
                    Err(e) => {
                        error.set(Some(format!("Auth failed: {}", e)));
                    }
                }
            });
        })
    };

    html! {
        <div class="auth-example">
            <h3>{"Authentication Example"}</h3>
            <button onclick={fetch_protected_data}>{"Fetch Protected Data"}</button>
            
            {if let Some(ref success) = *result {
                html! { <div class="success">{success}</div> }
            } else {
                html! {}
            }}
            
            {if let Some(ref err) = *error {
                html! { <div class="error">{format!("Error: {}", err)}</div> }
            } else {
                html! {}
            }}
        </div>
    }
}

/// Example using utility functions
#[function_component(UtilsExample)]
pub fn utils_example() -> Html {
    let result = use_state(|| None::<String>);
    let error = use_state(|| None::<String>);

    let test_utils = {
        let result = result.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let result = result.clone();
            let error = error.clone();
            
            spawn_local(async move {
                result.set(None);
                error.set(None);
                
                // Test utility functions
                match httpcalls::utils::get_json::<Vec<User>>("https://jsonplaceholder.typicode.com/users").await {
                    Ok(users) => {
                        log!("Utility function fetched {} users", users.len());
                        result.set(Some(format!("Fetched {} users using utility function", users.len())));
                    }
                    Err(e) => {
                        error.set(Some(format!("Utility function failed: {}", e)));
                    }
                }
            });
        })
    };

    html! {
        <div class="utils-example">
            <h3>{"Utility Functions Example"}</h3>
            <button onclick={test_utils}>{"Test Utilities"}</button>
            
            {if let Some(ref success) = *result {
                html! { <div class="success">{success}</div> }
            } else {
                html! {}
            }}
            
            {if let Some(ref err) = *error {
                html! { <div class="error">{format!("Error: {}", err)}</div> }
            } else {
                html! {}
            }}
        </div>
    }
}

/// Main component showcasing all HTTP client features
#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <div class="http-examples">
            <h1>{"HTTP Client Examples"}</h1>
            <p>{"Modern HTTP client with automatic state management integration"}</p>
            
            <div class="examples-grid">
                <GetExample />
                <PostExample />
                <UploadExample />
                <AuthExample />
                <UtilsExample />
            </div>
            
            <div class="features-info">
                <h2>{"Features Demonstrated"}</h2>
                <ul>
                    <li>{"✅ Fluent API with method chaining"}</li>
                    <li>{"✅ Automatic loader state management"}</li>
                    <li>{"✅ Progress tracking for uploads"}</li>
                    <li>{"✅ Automatic notifications on success/error"}</li>
                    <li>{"✅ JSON serialization/deserialization"}</li>
                    <li>{"✅ File uploads with FormData"}</li>
                    <li>{"✅ Custom headers and authentication"}</li>
                    <li>{"✅ Retry logic with exponential backoff"}</li>
                    <li>{"✅ Request timeouts"}</li>
                    <li>{"✅ Comprehensive error handling"}</li>
                </ul>
            </div>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
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

fn main() {
    yew::Renderer::<App>::new().render();
}