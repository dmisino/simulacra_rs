mod db;
use db.*;
use std::fs;
use serde::{Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::env;
use llm::openai::get_chat_completion;
use async_recursion::async_recursion;

#[derive(Debug, Deserialize)]
struct Step {
    name: String,
    #[serde(rename = "type")]
    step_type: String,
    file: Option<String>,
    results: Option<Vec<ResultValue>>,
}

#[derive(Debug, Deserialize)]
struct ResultValue {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Simulation {
    name: String,
    description: String,
    steps: Vec<Step>,
}

#[async_recursion]
pub async fn launch_simulation(simulation_name: &str) {
    let simulation_yaml = &get_file("simulation", format!("{}.yaml", simulation_name));
    let simulation: Simulation = serde_yaml::from_str(simulation_yaml).unwrap();
    let mut value_map: HashMap<&String, String> = HashMap::new();

    // Loop over the steps and execute. Steps can have output 
    // results, which are added to a hash map for use in later
    // steps. Step types:
    // - load: load a text file and add it to the value map, often
    //      used to load a piece of a prompt for an llm
    // - prompt: load a text file, replace any tags that match 
    //      existing values in the hash map, then prompt an llm 
    //      and add the result(s) to the hash map.
    // - agent: tbd 
    for step in &simulation.steps {
        match step.step_type.as_str() {
            "load" => {
                let mut file_contents = get_file(format!("simulation/{}", simulation_name), &step.file);
                // Replace any tags in the file
                for (key, value) in value_map.iter() {
                    let tag = format!("{{{}}}", key);
                    file_contents = file_contents.replace(tag.as_str(), value);
                }
                // A "load" step only has one result, so add it to the value map
                value_map.insert(&step.results[0].name, file_contents);
            },
            "prompt" => {
                let mut file_contents = get_file(format!("simulation/{}", simulation_name), &step.file);
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
                for result in &step.results {
                    // For each completion line, remove everything up to the first :
                    let mut result_value = completion_lines[index].to_string();
                    let mut parts: Vec<&str> = result_value.split(":").collect();
                    result_value = parts[1].trim().to_string();
                    // Add result to value map
                    value_map.insert(&result.name, result_value);
                    index = index + 1;
                }
            },
            "save_simulation" => {
                // Save simulation to database
                let simulation_id = save_simulation(&simulation.name).await;
                value_map.insert("siulation_id", simulation_id);
            },
            "save_world" => {
                // Save world to database
                let Some(world_name) = value_map.get("world_name");
                let Some(world_summary) = value_map.get("world_summary");
                let Some(world_description) = value_map.get("world_description");
                let Some(simulation_id) = value_map.get("simulation_id");
                let world_id = save_world(simulation_id, world_name, world_summary, world_description).await;
                value_map.insert("world_id", world_id);
            },
            "agent" => {
                println!("Agent step");
            },      
            _ => {
                println!("Unknown step type: {}", step.step_type);
            }
        }
    }
    if !value_map.is_empty() {
        // Print contents of value map
        println!("Results contained in value map: ");
        for (key, value) in value_map.iter() {
            println!("{}: {}", key, value);
        }
    }
}

pub fn get_file(folder: &str, file: &str) -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_dir = current_dir.join(folder);
    let file_path = file_dir.join(file);
    println!("get_file, file_path: {:?}", file_path);
    let result = fs::read_to_string(file_path).expect("Unable to read file");
    result
}