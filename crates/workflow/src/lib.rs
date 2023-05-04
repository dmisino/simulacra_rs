use std::fs;
use serde::{Deserialize};
use serde_yaml::{Result};

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

pub fn launch_workflow(workflow_file: &str) {
  // Load workflow file from path
  println!("Loading workflow file: {}", workflow_file);
  let yaml_str = &get_file(workflow_file);
  println!("Workflow file: {}", yaml_str);
  let workflow_result: Workflow = serde_yaml::from_str(yaml_str).unwrap();

  // Loop over the steps and execute
  for step in &workflow_result.steps {
      println!("Name: {}", step.name);
      println!("Type: {}", step.step_type);
      println!("File: {}", step.file);

      for result in &step.results {
          println!("Result: {}", result.name);
      }

      match(step.step_type.as_str()) {
          "workflow" => {
              println!("Workflow: {}", step.file);
              launch_workflow(&step.file);
          },
          "prompt_file" => {
              let file_contents = get_file(&step.file);
              println!("File: {}", file_contents);
          },
          "prompt_llm" => {
            let file_contents = get_file(&step.file);
            println!("LLM file: {}", file_contents);
        },          
          _ => {
              println!("Unknown step type: {}", step.step_type);
          }
      }
      println!();
  }
}

pub fn get_file(file_path: &str) -> String {
  match fs::read_to_string(file_path) {
      Ok(contents) => contents, //.replace("\r\n", "\\n"),
      Err(_) => "".to_string(),
  }
}