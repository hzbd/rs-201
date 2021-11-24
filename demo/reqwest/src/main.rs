use reqwest::header::{HeaderMap, HeaderName, USER_AGENT, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
// use serde_json::json;


#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    http_via: String,
    http_x_forwarded_for: String,
    client_ip: String,
    server: String,
    at: String,
}

#[tokio::main]
async fn main() {
    let url = format!("https://dadouqz.com/my");

    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
        headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(HeaderName::from_static("x-api-key"), HeaderValue::from_static("123-123-123"));
        headers
    }

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .headers(construct_headers())
        .send()
        .await
        .unwrap();
    println!("Success! {:?}", response);

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                Ok(parsed) => println!("Success! {:?}", parsed),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
}
