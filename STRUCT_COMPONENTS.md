# Using HttpCalls and HttpMessenger with Struct Components

This guide shows how to use the modern httpcalls and httpmessenger libraries with traditional Yew struct components.

## Quick Answer: Yes, You Can Use Struct Components!

The new libraries work perfectly with struct components. You have two main approaches:

1. **Automatic Integration** - Use `ContextConsumer` to get the dispatcher
2. **Manual Integration** - Create HTTP client without automatic state management

## Setup Requirements

### 1. Wrap Your App with StoreProvider

```rust
use httpmessenger::StoreProvider;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <StoreProvider>
                <YourMainComponent />
                <httpmessenger::Loader />
                <httpmessenger::NotificationManager />
            </StoreProvider>
        }
    }
}
```

### 2. Update Dependencies (If Needed)

Your existing `Cargo.toml` should work, but ensure you have:

```toml
[dependencies]
httpcalls = { path = "../httpcalls" }
httpmessenger = { path = "../httpmessenger" }
yew = "0.19" # or 0.20+
```

## Method 1: Automatic Integration (Recommended)

Use `ContextConsumer` to access the store dispatcher automatically.

```rust
use yew::prelude::*;
use httpmessenger::{StoreContext, AppAction, StoreDispatcher};
use httpcalls::{HttpClient, HttpError};
use wasm_bindgen_futures::spawn_local;

pub struct UserComponent {
    users: Vec<User>,
    error: Option<String>,
    dispatch: Option<StoreDispatcher>,
}

pub enum UserMessage {
    LoadUsers,
    UsersLoaded(Vec<User>),
    Error(String),
    StoreContextChanged(StoreContext),
}

impl Component for UserComponent {
    type Message = UserMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            users: Vec::new(),
            error: None,
            dispatch: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserMessage::LoadUsers => {
                if let Some(ref dispatch) = self.dispatch {
                    let link = ctx.link().clone();
                    let http_client = HttpClient::with_dispatcher(dispatch.clone());
                    
                    spawn_local(async move {
                        match http_client
                            .get("/api/users")
                            .with_loader(true)      // Automatic loader
                            .with_notifications(true) // Automatic notifications
                            .call_name("load_users")
                            .send()
                            .await
                        {
                            Ok(response) => {
                                if let Ok(users) = response.json::<Vec<User>>() {
                                    link.send_message(UserMessage::UsersLoaded(users));
                                }
                            }
                            Err(e) => {
                                link.send_message(UserMessage::Error(e.to_string()));
                            }
                        }
                    });
                }
                false
            }
            UserMessage::UsersLoaded(users) => {
                self.users = users;
                self.error = None;
                true
            }
            UserMessage::Error(error) => {
                self.error = Some(error);
                true
            }
            UserMessage::StoreContextChanged(context) => {
                self.dispatch = Some(context.dispatch);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let load_users = ctx.link().callback(|_| UserMessage::LoadUsers);

        html! {
            <ContextConsumer<StoreContext> context_callback={ctx.link().callback(UserMessage::StoreContextChanged)}>
                <div>
                    <button onclick={load_users}>{"Load Users"}</button>
                    // Render users and errors...
                </div>
            </ContextConsumer<StoreContext>>
        }
    }
}
```

## Method 2: Manual Integration

Create HTTP client without automatic state management and handle state manually.

```rust
use yew::prelude::*;
use httpmessenger::{StoreContext, AppAction};
use httpcalls::HttpClient;

pub struct ManualComponent {
    users: Vec<User>,
    is_loading: bool,
    dispatch: Option<StoreDispatcher>,
}

pub enum ManualMessage {
    LoadUsers,
    UsersLoaded(Vec<User>),
    StoreContextChanged(StoreContext),
}

impl Component for ManualComponent {
    type Message = ManualMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            users: Vec::new(),
            is_loading: false,
            dispatch: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ManualMessage::LoadUsers => {
                self.is_loading = true;
                
                // Manual loader control
                if let Some(ref dispatch) = self.dispatch {
                    dispatch.emit(AppAction::EnableLoader);
                }
                
                let link = ctx.link().clone();
                let dispatch = self.dispatch.clone();
                
                spawn_local(async move {
                    // Create client without automatic integration
                    let http_client = HttpClient::new();
                    
                    match http_client.get("/api/users").send().await {
                        Ok(response) => {
                            // Manual loader disable
                            if let Some(dispatch) = dispatch {
                                dispatch.emit(AppAction::DisableLoader);
                            }
                            
                            if let Ok(users) = response.json::<Vec<User>>() {
                                link.send_message(ManualMessage::UsersLoaded(users));
                            }
                        }
                        Err(_) => {
                            // Manual loader disable on error
                            if let Some(dispatch) = dispatch {
                                dispatch.emit(AppAction::DisableLoader);
                            }
                        }
                    }
                });
                
                true
            }
            ManualMessage::UsersLoaded(users) => {
                self.users = users;
                self.is_loading = false;
                true
            }
            ManualMessage::StoreContextChanged(context) => {
                self.dispatch = Some(context.dispatch);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let load_users = ctx.link().callback(|_| ManualMessage::LoadUsers);

        html! {
            <ContextConsumer<StoreContext> context_callback={ctx.link().callback(ManualMessage::StoreContextChanged)}>
                <div>
                    <button onclick={load_users} disabled={self.is_loading}>
                        {if self.is_loading { "Loading..." } else { "Load Users" }}
                    </button>
                    // Render users...
                </div>
            </ContextConsumer<StoreContext>>
        }
    }
}
```

## Common Patterns

### POST Request with JSON

```rust
// In your update method
UserMessage::CreateUser => {
    if let Some(ref dispatch) = self.dispatch {
        let link = ctx.link().clone();
        let http_client = HttpClient::with_dispatcher(dispatch.clone());
        
        spawn_local(async move {
            let user_data = User {
                name: "John".to_string(),
                email: "john@example.com".to_string(),
                age: 30,
            };
            
            match http_client
                .post("/api/users")
                .json(&user_data).unwrap()
                .with_loader(true)
                .send()
                .await
            {
                Ok(response) => {
                    link.send_message(UserMessage::UserCreated);
                }
                Err(e) => {
                    link.send_message(UserMessage::Error(e.to_string()));
                }
            }
        });
    }
    false
}
```

### File Upload

```rust
UserMessage::UploadFile => {
    if let Some(ref dispatch) = self.dispatch {
        let link = ctx.link().clone();
        let http_client = HttpClient::with_dispatcher(dispatch.clone());
        
        spawn_local(async move {
            let form_data = web_sys::FormData::new().unwrap();
            // Add file to form_data...
            
            match http_client
                .post("/api/upload")
                .form_data(form_data)
                .with_loader(true)
                .with_progress(true)  // Shows progress in global state
                .send()
                .await
            {
                Ok(_) => {
                    link.send_message(UserMessage::UploadComplete);
                }
                Err(e) => {
                    link.send_message(UserMessage::Error(e.to_string()));
                }
            }
        });
    }
    false
}
```

### Error Handling

```rust
match response {
    Ok(resp) => {
        if resp.is_success() {
            // Handle success
        } else {
            // Handle HTTP errors (4xx, 5xx)
            let error = format!("HTTP Error: {}", resp.status);
            link.send_message(UserMessage::Error(error));
        }
    }
    Err(HttpError::Network { message }) => {
        link.send_message(UserMessage::Error(format!("Network error: {}", message)));
    }
    Err(HttpError::Timeout) => {
        link.send_message(UserMessage::Error("Request timed out".to_string()));
    }
    Err(HttpError::Http { status, message, .. }) => {
        let error = format!("HTTP {}: {}", status, message);
        link.send_message(UserMessage::Error(error));
    }
    Err(e) => {
        link.send_message(UserMessage::Error(e.to_string()));
    }
}
```

## Best Practices for Struct Components

### 1. Store Dispatcher in Component State

```rust
pub struct MyComponent {
    // Your existing fields...
    dispatch: Option<StoreDispatcher>,
}
```

### 2. Use ContextConsumer in View

```rust
fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
        <ContextConsumer<StoreContext> context_callback={ctx.link().callback(MyMessage::StoreContextChanged)}>
            // Your component content
        </ContextConsumer<StoreContext>>
    }
}
```

### 3. Handle Context Changes

```rust
MyMessage::StoreContextChanged(context) => {
    self.dispatch = Some(context.dispatch);
    true
}
```

### 4. Clone for Async Operations

```rust
let link = ctx.link().clone();
let http_client = HttpClient::with_dispatcher(dispatch.clone());

spawn_local(async move {
    // Use cloned values in async block
});
```

### 5. Choose Integration Level

- **Automatic**: Use `HttpClient::with_dispatcher()` + `.with_loader(true)`
- **Manual**: Use `HttpClient::new()` + manual `dispatch.emit(AppAction::EnableLoader)`

## Migration from Old HttpAgent

### Before (Old Agent)

```rust
pub struct OldComponent {
    http_agent: Box<dyn Bridge<HttpAgent>>,
    users: Vec<User>,
}

impl Component for OldComponent {
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            http_agent: HttpAgent::bridge(
                ctx.link().callback(Self::Message::HttpResponse)
            ),
            users: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Message::LoadUsers => {
                let request = HttpAgentInput::build_get(
                    "load_users".to_string(),
                    "/api/users".to_string(),
                    true
                );
                self.http_agent.send(request);
                false
            }
            Message::HttpResponse(output) => {
                if output.call_name == "load_users" {
                    // Manual JSON parsing...
                }
                true
            }
        }
    }
}
```

### After (New System)

```rust
pub struct NewComponent {
    users: Vec<User>,
    dispatch: Option<StoreDispatcher>,
}

impl Component for NewComponent {
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            users: Vec::new(),
            dispatch: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::LoadUsers => {
                if let Some(ref dispatch) = self.dispatch {
                    let link = ctx.link().clone();
                    let http_client = HttpClient::with_dispatcher(dispatch.clone());
                    
                    spawn_local(async move {
                        match http_client
                            .get("/api/users")
                            .with_loader(true)
                            .call_name("load_users")
                            .send()
                            .await
                        {
                            Ok(response) => {
                                if let Ok(users) = response.json::<Vec<User>>() {
                                    link.send_message(Message::UsersLoaded(users));
                                }
                            }
                            Err(e) => {
                                // Handle error...
                            }
                        }
                    });
                }
                false
            }
            Message::UsersLoaded(users) => {
                self.users = users;
                true
            }
            Message::StoreContextChanged(context) => {
                self.dispatch = Some(context.dispatch);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ContextConsumer<StoreContext> context_callback={ctx.link().callback(Message::StoreContextChanged)}>
                // Your content
            </ContextConsumer<StoreContext>>
        }
    }
}
```

## Benefits with Struct Components

1. **Keep Your Existing Architecture** - No need to rewrite all components
2. **Gradual Migration** - Migrate components one by one
3. **Better Error Handling** - Structured error types
4. **Type-Safe JSON** - Automatic serialization/deserialization
5. **Automatic State Management** - Loader, progress, notifications
6. **Modern Async** - No more complex agent message handling

## Troubleshooting

### "ContextConsumer not found"

Make sure you import it:
```rust
use yew::prelude::*; // Includes ContextConsumer
```

### "StoreContext not in scope"

Import from httpmessenger:
```rust
use httpmessenger::StoreContext;
```

### "spawn_local not found"

Import from wasm_bindgen_futures:
```rust
use wasm_bindgen_futures::spawn_local;
```

### Component not re-rendering

Make sure to return `true` from `update()` when state changes:
```rust
Message::UsersLoaded(users) => {
    self.users = users;
    true  // Important!
}
```

## Conclusion

The new httpcalls and httpmessenger libraries work perfectly with struct components. You can:

- ✅ Keep your existing component architecture
- ✅ Get automatic state management integration
- ✅ Use modern HTTP client features
- ✅ Migrate gradually at your own pace
- ✅ Choose between automatic or manual integration

The migration provides significant benefits while preserving your current code structure.