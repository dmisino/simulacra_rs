use simulacra::{
  SimulacraError
};
//use std::fs::File;
use std::io::{Write};

#[tokio::main]
async fn main() -> Result<(), SimulacraError> {
  //let mut user_input = None;
  loop {
    // Get user input
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    //user_input = Some(input.trim().to_string());

    println!("{}", input.trim().to_owned());
  }
  Ok(())
}