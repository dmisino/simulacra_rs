extern crate db;
use db::datastore::*;
use std::fs;
use serde::{Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::env;
use llm::openai::get_chat_completion;
use async_recursion::async_recursion;
use std::fmt;

#[derive(Debug, Deserialize)]
struct Step {
    name: String,
    step_type: String,
    file: Option<String>,
    results: Option<Vec<ResultValue>>,
}

#[derive(Debug, Deserialize)]
struct ResultValue {
    name: String,
}

impl fmt::Display for ResultValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Deserialize)]
struct Simulation {
    name: String,
    description: String,
    steps: Vec<Step>,
}

#[async_recursion]
pub async fn launch_simulation(simulation_name: &str) -> i32 {
    let simulation_yaml = &get_file("simulation", &format!("{}.yaml", simulation_name));
    let simulation: Simulation = serde_yaml::from_str(simulation_yaml).unwrap();
    let mut value_map: HashMap<String, String> = HashMap::new();
    let mut new_simulation_id = 0;

    // Loop over the steps and execute. Steps can have output 
    // results, which are added to a hash map for use in later
    // steps.
    for step in simulation.steps {
        match step.step_type.as_str() {
            "load" => {
                let mut file_contents = get_file(&format!("simulation_files\\{}", simulation_name), &step.file.as_ref().unwrap());
                // Replace any tags in the file
                for (key, value) in value_map.iter() {
                    let tag = format!("{{{}}}", key);
                    file_contents = file_contents.replace(tag.as_str(), value);
                }
                // A "load" step only has one result, so add it to the value map
                if let Some(results) = step.results {
                    let result = results[0].to_string();
                    value_map.insert(result, file_contents);
                }
            },
            "prompt" => {
                let mut file_contents = get_file(&format!("simulation_files\\{}", simulation_name), &step.file.unwrap());
                // Replace any tags in the file
                for (key, value) in value_map.iter() {
                    let tag = format!("{{{}}}", key);
                    file_contents = file_contents.replace(tag.as_str(), value);
                }
                // For "prompt" steps, the file contains a prompt for an llm
                let completion = get_chat_completion(file_contents).await;
                
                // Split completion value by line breaks
                let mut completion_lines: Vec<&str> = completion.split("\n").collect();

                // Parse results and add to value map
                let mut index = 0;
                if let Some(results) = step.results {
                    for result in results.iter() {
                        // For each completion line, remove everything up to the first :
                        let mut line = completion_lines[index];
                        let mut parts: Vec<&str> = line.split(":").collect();
                        let result_value = parts[1].trim();
                        value_map.insert(result.to_string(), result_value.to_string());
                        index = index + 1;
                    }
                }
            },
            "save_simulation" => {
                // Save simulation to database
                let simulation_id = save_simulation(&simulation.name);
                //value_map.insert("simulation_id".to_string(), simulation_id.unwrap().to_string());
                //println!("New simulation created with id: {}", simulation_id.unwrap().to_string());
                if let Ok(id) = &simulation_id {
                    value_map.insert("simulation_id".to_string(), id.to_string());
                    new_simulation_id = id.to_string().parse::<i32>().unwrap();
                    //println!("New simulation created with id: {}", id.to_string());
                }
            },
            "save_world" => {
                // Save world to database
                let world_name = value_map.get("world_name").unwrap_or(&String::new()).to_string();
                let world_summary = value_map.get("world_summary").unwrap_or(&String::new()).to_string();
                let world_description = value_map.get("world_description").unwrap_or(&String::new()).to_string();
                let simulation_id = value_map.get("simulation_id").unwrap_or(&String::new()).to_string();
                let world_id = save_world(simulation_id.parse::<i32>().unwrap(), world_name, world_summary, world_description);
                value_map.insert("world_id".to_string(), world_id.unwrap().to_string());
            },
            "save_place" => {
                // Save place to database
                let place_name = value_map.get("place_name").unwrap_or(&String::new()).to_string();
                let place_summary = value_map.get("place_summary").unwrap_or(&String::new()).to_string();
                let place_description = value_map.get("place_description").unwrap_or(&String::new()).to_string();
                let world_id = value_map.get("world_id").unwrap_or(&String::new()).to_string();
                let place_id = save_place(world_id.parse::<i32>().unwrap(), place_name, place_summary, place_description);
                value_map.insert("place_id".to_string(), place_id.unwrap().to_string());
            },
            "save_npc" => {
                // Save npc to database
                let npc_name = value_map.get("npc_name").unwrap_or(&String::new()).to_string();
                let npc_summary = value_map.get("npc_summary").unwrap_or(&String::new()).to_string();
                let npc_description = value_map.get("npc_description").unwrap_or(&String::new()).to_string();
                let world_id = value_map.get("world_id").unwrap_or(&String::new()).to_string();
                let npc_id = save_npc(world_id.parse::<i32>().unwrap(), npc_name, npc_summary, npc_description);
                value_map.insert("npc_id".to_string(), npc_id.unwrap().to_string());
            },
            "agent" => {
                //println!("Agent step");
            },      
            _ => {
                println!("Unknown step type: {}", step.step_type);
            }
        }
    }
    new_simulation_id
}

pub fn get_file(folder: &str, file: &str) -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_dir = current_dir.join(folder);
    let file_path = file_dir.join(file);
    let result = fs::read_to_string(file_path).expect("Unable to read file");
    result
}