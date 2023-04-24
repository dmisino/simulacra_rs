mod openai;
use openai::{get_chat_completion};

#[tokio::main]
async fn main() {
  // Sample code, sends each line entered into console 
  // to chat completions and print response
  loop{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let response = get_chat_completion(line).await;
    println!("{}", response);
  }
}