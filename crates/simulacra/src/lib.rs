use prompt::*;
use llm::openai::*;

// Implement a pause duration between calls to the OpenAI API to prevent error 429 - Too many calls
pub fn pause() {
  std::thread::sleep(std::time::Duration::from_millis(1000));
}

pub fn status_message(content: &str) {
  println!("STATUS: {}", content);
}

pub fn story_message(content: &str) {
  println!("STORY: {}", content);
}

pub async fn get_new_world_place_npc() -> String {
  let prompt = get_prompt("world");
  let world: String = if !prompt.is_empty() {
      status_message("Generating world");
      get_chat_completion(prompt).await
  } else {
      "Error loading prompt from file".to_string()
  };
  pause();
  let prompt = get_prompt("place");
  let place: String = if !prompt.is_empty() {
      status_message("Generating place");
      get_chat_completion(prompt.replace("{world}", &world)).await
  } else {
      "Error loading prompt from file".to_string()
  };
  pause();
  let prompt = get_prompt("npc");
  let npc: String = if !prompt.is_empty() {
      status_message("Generating npc");    
      get_chat_completion(prompt.replace("{world_place}", &format!("{}\\n\\n{}", &world, &place))).await
  } else {
      "Error loading prompt from file".to_string()
  };
  pause();
  let world_place_npc = format!("\\n{}\\n\\n{}\\n\\n{}", &world, &place, &npc).replace("\\n", "\n");
  world_place_npc
}

pub async fn get_new_world() -> String {
  let prompt = get_prompt("world");
  let response = if !prompt.is_empty() {
      get_chat_completion(prompt).await
  } else {
      "Error loading prompt from file".to_string()
  };
  pause();
  response
}

pub async fn get_new_place(world: String) -> String {
  let prompt = get_prompt("place");
  let prompt = prompt.replace("{world}", &world);
  let response = if !prompt.is_empty() {
      get_chat_completion(prompt).await
  } else {
      "Error loading prompt from file".to_string()
  };
  pause();  
  response
}

pub async fn get_new_npc(world_place: String) -> String {
  let prompt = get_prompt("npc");
  let prompt = prompt.replace("{world_place}", &world_place);
  let response = if !prompt.is_empty() {
      get_chat_completion(prompt).await
  } else {
      "Error loading prompt from file".to_string()
  };
  pause();  
  response
}