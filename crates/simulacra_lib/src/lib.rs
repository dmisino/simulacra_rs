mod workflow;
use std::path::{Path, PathBuf};

use workflow::*;

// This is the main implementation of the simulacra framework. We
// will pass a workflow file to the workflow crate and let it run.
pub async fn start(workflow_file: String) {
    workflow::launch_workflow(&workflow_file).await;
    println!("Complete!");
}

// // Implement a pause duration between calls to the OpenAI API to prevent error 429 - Too many calls
// pub fn pause() {
//   std::thread::sleep(std::time::Duration::from_millis(1000));
// }
// pub async fn get_new_world_place_npc() -> (String, String, String) {
//   let prompt = get_prompt("world");
//   let world: String = if !prompt.is_empty() {
//       get_chat_completion(prompt).await
//   } else {
//       "Error loading prompt from file".to_string()
//   };
//   pause();
//   let prompt = get_prompt("place");
//   let place: String = if !prompt.is_empty() {
//       get_chat_completion(prompt.replace("{world}", &world)).await
//   } else {
//       "Error loading prompt from file".to_string()
//   };
//   pause();
//   let prompt = get_prompt("npc");
//   let npc: String = if !prompt.is_empty() {
//       get_chat_completion(prompt.replace("{world_place}", &format!("{}\\n\\n{}", &world, &place))).await
//   } else {
//       "Error loading prompt from file".to_string()
//   };
//   pause();
//   (world, place, npc)
// }

// pub async fn get_new_world() -> String {
//   let prompt = get_prompt("world");
//   let response = if !prompt.is_empty() {
//       get_chat_completion(prompt).await
//   } else {
//       "Error loading prompt from file".to_string()
//   };
//   pause();
//   response
// }

// pub async fn get_new_place(world: String) -> String {
//   let prompt = get_prompt("place");
//   let prompt = prompt.replace("{world}", &world);
//   let response = if !prompt.is_empty() {
//       get_chat_completion(prompt).await
//   } else {
//       "Error loading prompt from file".to_string()
//   };
//   pause();  
//   response
// }

// pub async fn get_new_npc(world_place: String) -> String {
//   let prompt = get_prompt("npc");
//   let prompt = prompt.replace("{world_place}", &world_place);
//   let response = if !prompt.is_empty() {
//       get_chat_completion(prompt).await
//   } else {
//       "Error loading prompt from file".to_string()
//   };
//   pause();  
//   response
// }