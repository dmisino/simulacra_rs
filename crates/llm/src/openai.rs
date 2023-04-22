use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: String,
    max_tokens: u64,
    n: u64,
    temperature: f64,
    frequency_penalty: f64,
    presence_penalty: f64,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    text: String,
}

pub async fn generate_completion(prompt: &str, api_key: &str) -> Option<String> {
    let url = "https://api.openai.com/v1/chat/completions";
    let model = "gpt-3.5-turbo";
    let request_body = OpenAIRequest {
        model: model.to_string(),
        prompt: prompt.to_owned(),
        max_tokens: 2000,
        n: 1,
        temperature: 0.7,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
    };
    let request_json = json!(request_body);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&request_json)
        .send()
        .await
        .ok()?;

    match response.status() {
        StatusCode::OK => (),
        status => {
          eprintln!("OpenAI API request failed with status code: {}", status);
          return None;
      }
    }

    let response_body: OpenAIResponse = response.json().await.ok()?;
    response_body.choices.get(0).map(|choice| choice.text.clone())
}
