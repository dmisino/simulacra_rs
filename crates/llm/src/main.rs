mod openai;
use openai::{generate_completion};

#[tokio::main]
async fn main() {
    let api_key = "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";
    let prompt = "How would you describe quantum mechanics to a computer programmer?";

    match generate_completion(prompt, api_key).await {
        Some(text) => println!("{}", text),
        None => println!("Failed to generate completion"),
    }
}