use reqwest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub message: Message,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    pub index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub async fn get_chat_completion(prompt: String) -> String {

  let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

  let mut data: String = r#"
  {
      "model": "gpt-3.5-turbo",
      "messages": [
        {
          "role": "system",
          "content": "You are an award winning writer who is helping as an assistant to write a new adventure story."
        },
        {
          "role": "user",
          "content": "{}"
        }
      ]
    }"#.to_string();
  data = format!("{}", data.replace("{}", prompt.trim_end()));

  let url = "https://api.openai.com/v1/chat/completions".to_string();
  let auth = format!("Bearer {}", api_key);

  let client = reqwest::Client::new();
  let response = client
      .post(url)
      .header(CONTENT_TYPE, "application/json")
      .header(AUTHORIZATION, auth)
      .body(data).send().await.unwrap();

    let chat_response = match response.status() {
      reqwest::StatusCode::OK => {
        match response.json::<Root>().await {
            Ok(parsed) => parsed.choices[0].message.content.to_owned(),
            Err(_) => "Error: unable to parse JSON response".to_owned(),
        }
      },
      reqwest::StatusCode::UNAUTHORIZED => "Error: Status 401 - Unauthorized".to_owned(),
      reqwest::StatusCode::TOO_MANY_REQUESTS => "Error: Status 429 - Too many requests".to_owned(),
      reqwest::StatusCode::BAD_REQUEST => "Error: Status 400 - Bad request".to_owned(),
      status => format!("Error: Unexpected HTTP status code from OpenAI API: {}", status.as_u16()),
  };
  chat_response
}

