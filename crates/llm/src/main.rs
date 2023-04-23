mod openai;
use openai::{get_chat_completion};

#[tokio::main]
async fn main() {
  // Sample code, just sends each line entered to chat completions and prints response to console
  loop{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let response = get_chat_completion(line).await;
    println!("{}", response);
  }
}