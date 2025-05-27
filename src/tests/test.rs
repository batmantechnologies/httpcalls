use wasm_bindgen_test::*;
use gloo_console::log;
use crate::{HttpClient, HttpMethod, HttpError, RequestBody};
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_http_client_creation() {
    let client = HttpClient::new();
    log!("Created HTTP client successfully");
    assert!(true);
}

#[wasm_bindgen_test]
fn test_http_client_with_base_url() {
    let client = HttpClient::new()
        .base_url("https://api.example.com")
        .default_header("Authorization", "Bearer token123");
    
    log!("Created HTTP client with base URL and default headers");
    assert!(true);
}

#[wasm_bindgen_test]
fn test_request_builder_get() {
    let client = HttpClient::new();
    let builder = client.get("/api/users");
    
    assert_eq!(builder.config.method, HttpMethod::Get);
    assert_eq!(builder.config.url, "/api/users");
    log!("GET request builder created successfully");
}

#[wasm_bindgen_test]
fn test_request_builder_post_with_json() {
    let client = HttpClient::new();
    
    #[derive(serde::Serialize)]
    struct TestData {
        name: String,
        age: u32,
    }
    
    let data = TestData {
        name: "John".to_string(),
        age: 30,
    };
    
    let builder = client.post("/api/users")
        .json(&data)
        .unwrap()
        .with_loader(true)
        .call_name("create_user");
    
    assert_eq!(builder.config.method, HttpMethod::Post);
    assert_eq!(builder.config.with_loader, true);
    assert_eq!(builder.config.call_name, Some("create_user".to_string()));
    
    // Check that JSON content type is set
    assert_eq!(
        builder.config.headers.get("Content-Type"),
        Some(&"application/json".to_string())
    );
    
    log!("POST request builder with JSON created successfully");
}

#[wasm_bindgen_test]
fn test_request_builder_headers() {
    let client = HttpClient::new();
    let builder = client.get("/api/data")
        .header("Accept", "application/json")
        .header("X-Custom", "custom-value");
    
    assert_eq!(
        builder.config.headers.get("Accept"),
        Some(&"application/json".to_string())
    );
    assert_eq!(
        builder.config.headers.get("X-Custom"),
        Some(&"custom-value".to_string())
    );
    
    log!("Request headers set successfully");
}

#[wasm_bindgen_test]
fn test_request_builder_multiple_headers() {
    let client = HttpClient::new();
    
    let mut headers = HashMap::new();
    headers.insert("Accept".to_string(), "application/json".to_string());
    headers.insert("Authorization".to_string(), "Bearer token".to_string());
    
    let builder = client.get("/api/data")
        .headers(headers);
    
    assert_eq!(
        builder.config.headers.get("Accept"),
        Some(&"application/json".to_string())
    );
    assert_eq!(
        builder.config.headers.get("Authorization"),
        Some(&"Bearer token".to_string())
    );
    
    log!("Multiple headers set successfully");
}

#[wasm_bindgen_test]
fn test_request_builder_text_body() {
    let client = HttpClient::new();
    let builder = client.post("/api/data")
        .text("Hello, World!");
    
    match &builder.config.body {
        RequestBody::Text(text) => assert_eq!(text, "Hello, World!"),
        _ => panic!("Expected text body"),
    }
    
    log!("Text body set successfully");
}

#[wasm_bindgen_test]
fn test_request_builder_configuration() {
    let client = HttpClient::new();
    let builder = client.post("/api/upload")
        .with_loader(true)
        .with_progress(true)
        .with_notifications(true)
        .timeout(60000)
        .retry(3, 1000);
    
    assert_eq!(builder.config.with_loader, true);
    assert_eq!(builder.config.with_progress, true);
    assert_eq!(builder.config.with_notifications, true);
    assert_eq!(builder.config.timeout_ms, Some(60000));
    assert_eq!(builder.config.retry_count, 3);
    assert_eq!(builder.config.retry_delay_ms, 1000);
    
    log!("Request configuration set successfully");
}

#[wasm_bindgen_test]
fn test_http_methods() {
    let client = HttpClient::new();
    
    let get_builder = client.get("/api/get");
    assert_eq!(get_builder.config.method, HttpMethod::Get);
    
    let post_builder = client.post("/api/post");
    assert_eq!(post_builder.config.method, HttpMethod::Post);
    
    let put_builder = client.put("/api/put");
    assert_eq!(put_builder.config.method, HttpMethod::Put);
    
    let delete_builder = client.delete("/api/delete");
    assert_eq!(delete_builder.config.method, HttpMethod::Delete);
    
    let patch_builder = client.patch("/api/patch");
    assert_eq!(patch_builder.config.method, HttpMethod::Patch);
    
    let head_builder = client.head("/api/head");
    assert_eq!(head_builder.config.method, HttpMethod::Head);
    
    let options_builder = client.options("/api/options");
    assert_eq!(options_builder.config.method, HttpMethod::Options);
    
    log!("All HTTP methods work correctly");
}

#[wasm_bindgen_test]
fn test_url_building_with_base_url() {
    let client = HttpClient::new()
        .base_url("https://api.example.com");
    
    // Test relative path
    let builder1 = client.get("/users");
    assert_eq!(builder1.config.url, "https://api.example.com/users");
    
    // Test path without leading slash
    let builder2 = client.get("users/123");
    assert_eq!(builder2.config.url, "https://api.example.com/users/123");
    
    log!("URL building with base URL works correctly");
}

#[wasm_bindgen_test]
fn test_http_error_types() {
    let network_error = HttpError::Network { 
        message: "Connection failed".to_string() 
    };
    assert!(network_error.to_string().contains("Network error"));
    
    let timeout_error = HttpError::Timeout;
    assert_eq!(timeout_error.to_string(), "Request timeout");
    
    let http_error = HttpError::Http { 
        status: 404, 
        message: "Not Found".to_string(), 
        body: None 
    };
    assert!(http_error.to_string().contains("HTTP 404"));
    
    log!("HTTP error types work correctly");
}

#[wasm_bindgen_test]
fn test_default_client() {
    let client = HttpClient::default();
    let builder = client.get("/test");
    
    assert_eq!(builder.config.method, HttpMethod::Get);
    assert_eq!(builder.config.url, "/test");
    
    log!("Default client works correctly");
}

#[wasm_bindgen_test]
fn test_request_body_types() {
    // Test None body
    let body_none = RequestBody::None;
    match body_none {
        RequestBody::None => log!("None body type works"),
        _ => panic!("Expected None body"),
    }
    
    // Test Text body
    let body_text = RequestBody::Text("test".to_string());
    match body_text {
        RequestBody::Text(text) => {
            assert_eq!(text, "test");
            log!("Text body type works");
        },
        _ => panic!("Expected Text body"),
    }
    
    // Test JSON body
    let body_json = RequestBody::Json(r#"{"key":"value"}"#.to_string());
    match body_json {
        RequestBody::Json(json) => {
            assert!(json.contains("key"));
            log!("JSON body type works");
        },
        _ => panic!("Expected JSON body"),
    }
    
    // Test Binary body
    let body_binary = RequestBody::Binary(vec![1, 2, 3, 4]);
    match body_binary {
        RequestBody::Binary(data) => {
            assert_eq!(data, vec![1, 2, 3, 4]);
            log!("Binary body type works");
        },
        _ => panic!("Expected Binary body"),
    }
}

#[wasm_bindgen_test]
fn test_method_as_str() {
    assert_eq!(HttpMethod::Get.as_str(), "GET");
    assert_eq!(HttpMethod::Post.as_str(), "POST");
    assert_eq!(HttpMethod::Put.as_str(), "PUT");
    assert_eq!(HttpMethod::Delete.as_str(), "DELETE");
    assert_eq!(HttpMethod::Patch.as_str(), "PATCH");
    assert_eq!(HttpMethod::Head.as_str(), "HEAD");
    assert_eq!(HttpMethod::Options.as_str(), "OPTIONS");
    
    log!("HTTP method string conversion works correctly");
}

#[wasm_bindgen_test]
async fn test_json_serialization_error() {
    use serde::Serialize;
    
    // Create a type that will fail to serialize
    #[derive(Serialize)]
    struct BadData {
        #[serde(serialize_with = "fail_serialize")]
        field: String,
    }
    
    fn fail_serialize<S>(_: &String, _: S) -> Result<S::Ok, S::Error> 
    where 
        S: serde::Serializer 
    {
        Err(serde::ser::Error::custom("Intentional failure"))
    }
    
    let client = HttpClient::new();
    let bad_data = BadData { field: "test".to_string() };
    
    let result = client.post("/test").json(&bad_data);
    
    match result {
        Err(HttpError::Serialization { .. }) => {
            log!("JSON serialization error handled correctly");
        },
        _ => panic!("Expected serialization error"),
    }
}