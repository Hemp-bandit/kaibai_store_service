pub mod store_err;
pub mod structs;

#[macro_export]
macro_rules! http_client {
    () => {{
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_str("application/json;charset=utf8").unwrap(),
        );
        headers.insert(
            "service_call",
            reqwest::header::HeaderValue::from_str("store_service").unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("build client faille");
        client
    }};
}
