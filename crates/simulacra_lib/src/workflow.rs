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
    file: String,
    results: Vec<ResultValue>,
}

#[derive(Debug, Deserialize)]
struct ResultValue {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Workflow {
    name: String,
    description: String,
    steps: Vec<Step>,
}

#[async_recursion]
pub async fn launch_workflow(workflow_file: &str) {
    let workflow_yaml = &get_file("workflow", &workflow_file);
    let workflow: Workflow = serde_yaml::from_str(workflow_yaml).unwrap();
    let mut value_map: HashMap<&String, String> = HashMap::new();

    // Loop over the steps and execute
    for step in &workflow.steps {
        match step.step_type.as_str() {
            "workflow" => {
                launch_workflow(&step.file).await;
            },
            "load" => {
                // Load the file                 
                let mut file_contents = get_file("prompt", &step.file);
                // Replace any tags in the file
                for (key, value) in value_map.iter() {
                    let tag = format!("{{{}}}", key);
                    file_contents = file_contents.replace(tag.as_str(), value);
                }
                // A "load" step only has one result, so add it to the value map
                value_map.insert(&step.results[0].name, file_contents);
            },
            "prompt" => {
                // Load the file
                let mut file_contents = get_file("prompt", &step.file);
                
                // Replace any tags in the file
                for (key, value) in value_map.iter() {
                    let tag = format!("{{{}}}", key);
                    file_contents = file_contents.replace(tag.as_str(), value);
                }

                // For "prompt" steps, the file contains a prompt for an llm
                println!("step_name: {}, prompt: \n\n{}\n\n", &step.name, file_contents);
                let completion = get_chat_completion(file_contents).await;
                
                // Split completion value by line breaks
                let mut completion_lines: Vec<&str> = completion.split("\n").collect();

                // Parse results and add to value map
                for result in &step.results {
                    // For each completion line, remove everything up to the first :
                    let mut result_value = completion_lines[0].to_string();
                    let mut parts: Vec<&str> = result_value.split(":").collect();
                    result_value = parts[1].trim().to_string();
                    // Add result to value map
                    value_map.insert(&result.name, result_value);
                }
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