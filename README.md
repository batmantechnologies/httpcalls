# HttpCalls - Modern HTTP Client for Yew

A modern, fluent HTTP client for Yew applications with automatic state management integration via `httpmessenger`. This library provides a comprehensive solution for making HTTP requests with automatic loader states, progress tracking, notifications, and robust error handling.

## Features

- 🚀 **Fluent API**: Chain method calls for readable request building
- 🔄 **Automatic State Management**: Loader states, progress, and notifications via httpmessenger
- 🎯 **Type Safety**: Comprehensive error types and compile-time checking
- ⚡ **Modern Async**: Built on async/await patterns with futures
- 📁 **Multiple Content Types**: JSON, FormData, raw text, and binary support
- 🔧 **Request Middleware**: Headers, authentication, and request interception
- ⏱️ **Timeout Support**: Per-request timeout configuration
- 🔁 **Retry Logic**: Automatic retry with exponential backoff
- 📊 **Upload Progress**: Real-time upload progress tracking
- 🎨 **Theme Integration**: Works seamlessly with httpmessenger themes
- 📱 **WASM Optimized**: Designed specifically for WebAssembly targets

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
httpcalls = "0.2"
httpmessenger = "0.2"
yew = { version = "0.20", features = ["csr"] }
```

## Quick Start

### Basic Setup

```rust
use yew::prelude::*;
use httpmessenger::StoreProvider;
use httpcalls::{HttpClient, use_http_client};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <StoreProvider>
            <MyComponent />
        </StoreProvider>
    }
}

#[function_component(MyComponent)]
pub fn my_component() -> Html {
    let http_client = use_http_client(); // Automatic state integration
    
    // Your component logic
    html! { <div>{"My Component"}</div> }
}
```

### Simple GET Request

```rust
use httpcalls::HttpClient;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[function_component(UsersList)]
pub fn users_list() -> Html {
    let users = use_state(|| Vec::<User>::new());
    let http_client = use_http_client();

    let fetch_users = {
        let users = users.clone();
        let http_client = http_client.clone();
        
        Callback::from(move |_| {
            let users = users.clone();
            let http_client = http_client.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                match http_client
                    .get("https://jsonplaceholder.typicode.com/users")
                    .with_loader(true)  // Automatic loader state
                    .call_name("fetch_users")
                    .send()
                    .await
                {
                    Ok(response) => {
                        if let Ok(user_list) = response.json::<Vec<User>>() {
                            users.set(user_list);
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to fetch users: {}", e);
                    }
                }
            });
        })
    };

    html! {
        <div>
            <button onclick={fetch_users}>{"Load Users"}</button>
            // Render users...
        </div>
    }
}
```

## API Reference

### HttpClient

The main HTTP client with fluent API:

```rust
let client = HttpClient::new()
    .base_url("https://api.example.com")
    .default_header("Authorization", "Bearer token")
    .default_timeout(30000);
```

#### Methods

- `new()` - Create a new HTTP client
- `with_dispatcher(dispatch)` - Create client with state dispatcher
- `base_url(url)` - Set base URL for all requests
- `default_header(name, value)` - Add default header
- `default_timeout(ms)` - Set default timeout

#### HTTP Methods

```rust
client.get("/api/users")         // GET request
client.post("/api/users")        // POST request
client.put("/api/users/1")       // PUT request
client.delete("/api/users/1")    // DELETE request
client.patch("/api/users/1")     // PATCH request
client.head("/api/users")        // HEAD request
client.options("/api/users")     // OPTIONS request
```

### RequestBuilder

Fluent request builder with comprehensive configuration:

```rust
let response = client
    .post("/api/users")
    .json(&user_data)?              // JSON body
    .header("Accept", "application/json")  // Custom header
    .with_loader(true)              // Enable loader state
    .with_progress(true)            // Enable progress tracking
    .with_notifications(true)       // Enable success/error notifications
    .call_name("create_user")       // Set call name for tracking
    .timeout(60000)                 // 60 second timeout
    .retry(3, 1000)                 // Retry 3 times with 1s delay
    .send()
    .await?;
```

#### Configuration Methods

- `header(name, value)` - Add single header
- `headers(map)` - Add multiple headers
- `json(data)` - Set JSON body with automatic Content-Type
- `form_data(form)` - Set FormData body for file uploads
- `text(content)` - Set plain text body
- `binary(data)` - Set binary data body
- `with_loader(enabled)` - Enable/disable automatic loader
- `with_progress(enabled)` - Enable/disable progress tracking
- `with_notifications(enabled)` - Enable/disable notifications
- `call_name(name)` - Set call name for tracking
- `timeout(ms)` - Set request timeout
- `no_timeout()` - Disable timeout
- `retry(count, delay_ms)` - Configure retry behavior

### HttpResponse

Response wrapper with utility methods:

```rust
let response = client.get("/api/data").send().await?;

// Parse JSON
let data: MyData = response.json()?;

// Get text content
let text = response.text();

// Check status
if response.is_success() {
    // Handle 2xx response
}

// Get headers
if let Some(content_type) = response.header("content-type") {
    // Use content type
}

// Response properties
println!("Status: {}", response.status);
println!("URL: {}", response.url);
println!("Redirected: {}", response.redirected);
```

### Error Handling

Comprehensive error types for robust error handling:

```rust
match client.get("/api/data").send().await {
    Ok(response) => {
        // Handle success
    }
    Err(HttpError::Network { message }) => {
        // Network connectivity issues
    }
    Err(HttpError::Timeout) => {
        // Request timed out
    }
    Err(HttpError::Http { status, message, body }) => {
        // HTTP error status (4xx, 5xx)
        match status {
            404 => println!("Not found"),
            500 => println!("Server error"),
            _ => println!("HTTP error: {}", status),
        }
    }
    Err(HttpError::Serialization { message }) => {
        // JSON parsing errors
    }
    Err(e) => {
        // Other errors
        println!("Error: {}", e);
    }
}
```

#### Error Types

- `Network { message }` - Network connectivity issues
- `Timeout` - Request timeout
- `InvalidUrl { url }` - Malformed URL
- `Serialization { message }` - JSON serialization/parsing errors
- `Http { status, message, body }` - HTTP error responses
- `Cancelled` - Request was cancelled
- `InvalidResponse` - Malformed response
- `Configuration { message }` - Client configuration errors

## Advanced Usage

### Authentication & Headers

```rust
// Client with authentication
let client = HttpClient::new()
    .base_url("https://api.example.com")
    .default_header("Authorization", "Bearer your-token")
    .default_header("X-API-Version", "v1");

// Request with additional headers
let response = client
    .get("/protected/data")
    .header("Accept", "application/json")
    .header("X-Request-ID", "unique-id")
    .send()
    .await?;
```

### File Upload with Progress

```rust
#[function_component(FileUpload)]
pub fn file_upload() -> Html {
    let http_client = use_http_client();
    
    let upload_file = {
        let http_client = http_client.clone();
        
        Callback::from(move |file_data: Vec<u8>| {
            let http_client = http_client.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let form_data = web_sys::FormData::new().unwrap();
                let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
                    &js_sys::Array::of1(&js_sys::Uint8Array::from(&file_data[..])),
                    web_sys::BlobPropertyBag::new().type_("image/jpeg"),
                ).unwrap();
                
                form_data.append_with_blob_and_filename("file", &blob, "image.jpg").unwrap();
                
                match http_client
                    .post("/api/upload")
                    .form_data(form_data)
                    .with_loader(true)
                    .with_progress(true)  // Shows progress in httpmessenger
                    .timeout(120000)      // 2 minute timeout for upload
                    .send()
                    .await
                {
                    Ok(response) => {
                        log::info!("Upload successful: {}", response.status);
                    }
                    Err(e) => {
                        log::error!("Upload failed: {}", e);
                    }
                }
            });
        })
    };

    html! {
        <div>
            <input type="file" onchange={/* handle file selection */} />
            <button onclick={move |_| upload_file.emit(vec![])}>{"Upload"}</button>
        </div>
    }
}
```

### Retry Logic & Error Recovery

```rust
let response = client
    .get("/api/unreliable-endpoint")
    .with_loader(true)
    .retry(3, 1000)  // Retry 3 times with 1 second delay
    .timeout(10000)  // 10 second timeout per attempt
    .send()
    .await?;
```

### Custom Client Configuration

```rust
// Create specialized client for different APIs
let auth_client = HttpClient::new()
    .base_url("https://auth.example.com")
    .default_header("Content-Type", "application/json")
    .default_timeout(15000);

let api_client = HttpClient::new()
    .base_url("https://api.example.com")
    .default_header("Authorization", "Bearer token")
    .default_header("Accept", "application/json")
    .default_timeout(30000);
```

## Utility Functions

Convenience functions for common operations:

```rust
use httpcalls::utils;

// Simple JSON GET
let users: Vec<User> = utils::get_json("https://api.example.com/users").await?;

// Simple JSON POST
let created_user: User = utils::post_json("/api/users", &new_user).await?;

// File upload with progress
let response = utils::upload_file(
    "/api/upload",
    &file_data,
    "document.pdf",
    "application/pdf",
    true,  // with_progress
).await?;

// File download
let file_data = utils::download_file("/api/files/123").await?;
```

## Integration with HttpMessenger

The HTTP client automatically integrates with `httpmessenger` for state management:

### Automatic Loader States

```rust
// Enables global loader automatically
let response = client
    .get("/api/data")
    .with_loader(true)  // httpmessenger loader will be active
    .send()
    .await?;
// Loader automatically disabled when request completes
```

### Progress Tracking

```rust
// Updates global progress state
let response = client
    .post("/api/upload")
    .form_data(form_data)
    .with_progress(true)  // Updates httpmessenger progress (0.0-1.0)
    .send()
    .await?;
```

### Automatic Notifications

```rust
// Shows success/error notifications
let response = client
    .post("/api/users")
    .json(&user_data)?
    .with_notifications(true)  // Shows httpmessenger notifications
    .send()
    .await?;
```

## Migration from Legacy HTTP Agent

### Before (Old yew-agent approach)

```rust
// Old agent-based approach
let http_agent = HttpAgent::bridge(ctx.link().callback(Message::HttpResponse));
let request = HttpAgentInput::build_get("fetch_users", "/api/users", true);
http_agent.send(request);
```

### After (Modern fluent API)

```rust
// New fluent API approach
let http_client = use_http_client();

wasm_bindgen_futures::spawn_local(async move {
    let response = http_client
        .get("/api/users")
        .with_loader(true)
        .call_name("fetch_users")
        .send()
        .await?;
    
    // Handle response directly
});
```

## Performance & Best Practices

### 1. Use the Hook for Automatic State Management

```rust
// ✅ Good - automatic integration
let http_client = use_http_client();

// ❌ Less optimal - manual state management
let http_client = HttpClient::new();
```

### 2. Reuse Clients with Base Configuration

```rust
// ✅ Good - reuse configured client
let api_client = HttpClient::new()
    .base_url("https://api.example.com")
    .default_header("Authorization", "Bearer token");

// Use api_client for multiple requests
```

### 3. Handle Errors Appropriately

```rust
// ✅ Good - specific error handling
match response {
    Ok(data) => { /* handle success */ }
    Err(HttpError::Network { .. }) => { /* retry or show connection error */ }
    Err(HttpError::Http { status: 401, .. }) => { /* redirect to login */ }
    Err(HttpError::Http { status: 404, .. }) => { /* show not found */ }
    Err(e) => { /* general error handling */ }
}
```

### 4. Use Appropriate Timeouts

```rust
// ✅ Good - appropriate timeouts for different operations
client.get("/api/quick").timeout(5000);     // 5s for fast endpoints
client.post("/api/upload").timeout(120000); // 2min for uploads
client.get("/api/report").timeout(60000);   // 1min for slow operations
```

## Browser Support

- All modern browsers supporting WebAssembly
- Progressive enhancement with feature detection
- Automatic fallbacks for older fetch implementations
- No additional polyfills required

## Testing

The library includes comprehensive tests:

```bash
# Run tests in browser
wasm-pack test --headless --firefox

# Run tests with console output
wasm-pack test --headless --chrome
```

### Writing Tests

```rust
use httpcalls::HttpClient;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_get_request() {
    let client = HttpClient::new();
    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await
        .unwrap();
    
    assert!(response.is_success());
}
```

## Examples

See the `examples/` directory for comprehensive usage examples:

- `basic_usage.rs` - Complete examples of all features
- GET/POST requests with JSON
- File uploads with progress
- Authentication patterns
- Error handling strategies

## Contributing

Contributions are welcome! Please see our contributing guidelines for details.

## License

This project is licensed under the MIT OR Apache-2.0 license.

---

**Note**: This library requires `httpmessenger` for automatic state management. For standalone HTTP functionality without state integration, consider using `reqwasm` directly.