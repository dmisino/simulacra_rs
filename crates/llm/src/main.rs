mod openai;
use openai::{generate_completion};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = match env::var_os("OPENAI_API_KEY") {
      Some(api_key) => api_key.into_string().unwrap(),
      None => panic!("OPENAI_API_KEY is not set")
    };  
    let prompt = "How would you describe quantum mechanics to a computer programmer?";

    match generate_completion(prompt, api_key).await {
        Some(text) => println!("{}", text),
        None => println!("Failed to generate completion"),
    }
}