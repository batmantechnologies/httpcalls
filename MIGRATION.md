# Migration Guide: From yew-agent HttpAgent to Modern HTTP Client

This guide will help you migrate your existing `httpcalls` implementation from the old `yew-agent` based system to the new modern HTTP client with automatic state management.

## Quick Summary

The new version provides:
- ✅ Modern Yew 0.20+ compatibility
- ✅ Fluent API for readable request building
- ✅ Automatic state management integration with httpmessenger
- ✅ Better error handling with typed errors
- ✅ Progress tracking and notifications
- ✅ Retry logic with exponential backoff
- ✅ Request timeout configuration

## Step-by-Step Migration

### 1. Update Your Dependencies

**Old `Cargo.toml`:**
```toml
[dependencies]
yew-agent = "0.1"
httpcalls = "0.1"
httpmessenger = "0.1"
```

**New `Cargo.toml`:**
```toml
[dependencies]
httpcalls = "0.2"
httpmessenger = "0.2"
yew = { version = "0.20", features = ["csr"] }
```

### 2. Replace Agent Bridges with HTTP Client

**Old approach:**
```rust
pub struct MyComponent {
    http_agent: Box<dyn Bridge<HttpAgent>>,
    link: ComponentLink<Self>,
}

impl Component for MyComponent {
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            http_agent: HttpAgent::bridge(
                ctx.link().callback(Self::Message::HttpResponse)
            ),
            link: ctx.link().clone(),
        }
    }
}
```

**New approach:**
```rust
#[function_component(MyComponent)]
pub fn my_component() -> Html {
    let http_client = use_http_client(); // Automatic state integration
    // No bridge management needed!
}
```

### 3. Update Request Building

**Old message-based approach:**
```rust
pub enum Message {
    HttpResponse(HttpAgentOutput),
    FetchData,
}

fn update(&mut self, msg: Self::Message) -> bool {
    match msg {
        Message::FetchData => {
            let request = HttpAgentInput::build_get(
                "fetch_data".to_string(),
                "/api/data".to_string(),
                true // loader
            );
            self.http_agent.send(request);
            false
        }
        Message::HttpResponse(output) => {
            if output.call_name == "fetch_data" {
                // Handle response
                match output.status_code {
                    200 => {
                        if let Some(data) = output.value {
                            // Parse data manually
                        }
                    }
                    _ => {
                        // Handle error
                    }
                }
            }
            true
        }
    }
}
```

**New fluent API approach:**
```rust
let fetch_data = {
    let http_client = http_client.clone();
    Callback::from(move |_| {
        let http_client = http_client.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            match http_client
                .get("/api/data")
                .with_loader(true)
                .call_name("fetch_data")
                .send()
                .await
            {
                Ok(response) => {
                    // Type-safe JSON parsing
                    if let Ok(data) = response.json::<MyDataType>() {
                        // Handle success
                    }
                }
                Err(e) => {
                    // Structured error handling
                    match e {
                        HttpError::Network { .. } => { /* Network error */ }
                        HttpError::Http { status, .. } => { /* HTTP error */ }
                        _ => { /* Other errors */ }
                    }
                }
            }
        });
    })
};
```

### 4. Update POST Requests with JSON

**Old approach:**
```rust
let data = json!({"name": "John", "age": 30}).to_string();
let request = HttpAgentInput::build_post(
    "create_user".to_string(),
    "/api/users".to_string(),
    data,
    true
);
self.http_agent.send(request);
```

**New approach:**
```rust
#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}

let user = User { name: "John".to_string(), age: 30 };

let response = http_client
    .post("/api/users")
    .json(&user)?
    .with_loader(true)
    .with_notifications(true)
    .call_name("create_user")
    .send()
    .await?;
```

### 5. Update File Upload

**Old approach:**
```rust
let request = HttpAgentInput::build_form(
    "upload_file".to_string(),
    "/api/upload".to_string(),
    form_data,
    true
);
self.http_agent.send(request);
```

**New approach:**
```rust
let response = http_client
    .post("/api/upload")
    .form_data(form_data)
    .with_loader(true)
    .with_progress(true)  // Automatic progress tracking
    .timeout(120000)      // 2 minute timeout
    .retry(3, 1000)       // Retry 3 times with 1s delay
    .call_name("upload_file")
    .send()
    .await?;
```

## Complete Migration Example

**Before (Old yew-agent approach):**
```rust
use yew::prelude::*;
use httpcalls::{HttpAgent, HttpAgentInput, HttpAgentOutput};
use yew_agent::{Bridge, Bridged};

pub struct UserManager {
    http_agent: Box<dyn Bridge<HttpAgent>>,
    users: Vec<User>,
    loading: bool,
}

pub enum Message {
    LoadUsers,
    HttpResponse(HttpAgentOutput),
}

impl Component for UserManager {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            http_agent: HttpAgent::bridge(
                ctx.link().callback(Self::Message::HttpResponse)
            ),
            users: Vec::new(),
            loading: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::LoadUsers => {
                self.loading = true;
                let request = HttpAgentInput::build_get(
                    "load_users".to_string(),
                    "/api/users".to_string(),
                    false
                );
                self.http_agent.send(request);
                true
            }
            Message::HttpResponse(output) => {
                self.loading = false;
                if output.call_name == "load_users" && output.status_code == 200 {
                    if let Some(data) = output.value {
                        // Manual JSON parsing
                        if let Ok(users) = serde_json::from_str::<Vec<User>>(&data) {
                            self.users = users;
                        }
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let load_users = ctx.link().callback(|_| Message::LoadUsers);

        html! {
            <div>
                <button onclick={load_users} disabled={self.loading}>
                    {if self.loading { "Loading..." } else { "Load Users" }}
                </button>
                // Render users...
            </div>
        }
    }
}
```

**After (New HTTP client approach):**
```rust
use yew::prelude::*;
use httpcalls::{HttpClient, use_http_client};
use httpmessenger::StoreProvider;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[function_component(UserManager)]
pub fn user_manager() -> Html {
    let users = use_state(|| Vec::<User>::new());
    let http_client = use_http_client(); // Automatic state integration

    let load_users = {
        let users = users.clone();
        let http_client = http_client.clone();
        
        Callback::from(move |_| {
            let users = users.clone();
            let http_client = http_client.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                match http_client
                    .get("/api/users")
                    .with_loader(true)      // Automatic loader state
                    .with_notifications(true) // Automatic notifications
                    .call_name("load_users")
                    .send()
                    .await
                {
                    Ok(response) => {
                        if let Ok(user_list) = response.json::<Vec<User>>() {
                            users.set(user_list);
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to load users: {}", e);
                    }
                }
            });
        })
    };

    html! {
        <div>
            <button onclick={load_users}>{"Load Users"}</button>
            // Render users... (loading state handled automatically)
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <StoreProvider>
            <UserManager />
        </StoreProvider>
    }
}
```

## Benefits of Migration

### 1. Reduced Boilerplate
- **Before**: ~80 lines for basic HTTP component
- **After**: ~30 lines for the same functionality

### 2. Better Developer Experience
- Type-safe JSON serialization/deserialization
- Automatic error handling with structured error types
- Fluent API with method chaining
- No manual bridge management

### 3. Enhanced Features
- Automatic loader state management
- Progress tracking for uploads
- Request retry with exponential backoff
- Configurable timeouts
- Global notifications

### 4. Better Error Handling
```rust
// Old way - manual status checking
if output.status_code == 200 {
    // Success
} else {
    // Generic error handling
}

// New way - structured error types
match result {
    Ok(response) => { /* Success */ }
    Err(HttpError::Network { message }) => { /* Network error */ }
    Err(HttpError::Http { status: 401, .. }) => { /* Unauthorized */ }
    Err(HttpError::Http { status: 404, .. }) => { /* Not found */ }
    Err(HttpError::Timeout) => { /* Timeout */ }
    Err(e) => { /* Other errors */ }
}
```

## Common Migration Patterns

### 1. Converting GET Requests
```rust
// Old
let request = HttpAgentInput::build_get("fetch_data", "/api/data", true);
http_agent.send(request);

// New
let response = http_client
    .get("/api/data")
    .with_loader(true)
    .call_name("fetch_data")
    .send()
    .await?;
```

### 2. Converting POST Requests
```rust
// Old
let data = serde_json::to_string(&payload)?;
let request = HttpAgentInput::build_post("create_item", "/api/items", data, true);
http_agent.send(request);

// New
let response = http_client
    .post("/api/items")
    .json(&payload)?
    .with_loader(true)
    .call_name("create_item")
    .send()
    .await?;
```

### 3. Converting File Uploads
```rust
// Old
let request = HttpAgentInput::build_form("upload", "/api/upload", form_data, true);
http_agent.send(request);

// New
let response = http_client
    .post("/api/upload")
    .form_data(form_data)
    .with_loader(true)
    .with_progress(true)
    .call_name("upload")
    .send()
    .await?;
```

### 4. Converting DELETE Requests
```rust
// Old
let request = HttpAgentInput::build_delete("delete_item", "/api/items/123", true);
http_agent.send(request);

// New
let response = http_client
    .delete("/api/items/123")
    .with_loader(true)
    .call_name("delete_item")
    .send()
    .await?;
```

## Troubleshooting

### Common Issues

1. **"use_http_client must be used within a StoreProvider"**
   - Make sure your app is wrapped with `<StoreProvider>`

2. **Compilation errors with async/await**
   - Use `wasm_bindgen_futures::spawn_local` for async operations
   - Ensure you're using function components

3. **Type errors with JSON serialization**
   - Implement `Serialize` and `Deserialize` for your data types
   - Use `#[derive(Serialize, Deserialize)]`

4. **State not updating automatically**
   - Use `use_http_client()` instead of `HttpClient::new()` for automatic state integration

### Performance Tips

1. **Reuse HTTP client instances** when possible
2. **Use specific error handling** instead of generic catch-all
3. **Enable progress tracking** only for long-running operations
4. **Set appropriate timeouts** for different types of requests

## Migration Checklist

- [ ] Update dependencies in `Cargo.toml`
- [ ] Replace `HttpAgent` bridges with `use_http_client()`
- [ ] Convert struct components to function components
- [ ] Replace message-based requests with async/await
- [ ] Update JSON handling to use type-safe serialization
- [ ] Add error handling with structured error types
- [ ] Wrap app with `<StoreProvider>`
- [ ] Test all HTTP operations
- [ ] Remove old yew-agent imports

## Next Steps

1. Start with simple GET requests
2. Gradually migrate POST/PUT/DELETE operations
3. Add file upload functionality if needed
4. Implement proper error handling
5. Add progress tracking for appropriate operations
6. Remove old HttpAgent code once migration is complete

The migration provides significant benefits in terms of code clarity, type safety, and developer experience while maintaining all the functionality of the original system.