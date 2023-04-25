use simulacra::*;
use std::io::{self, Write};

fn print_console(message: &str) {
  println!("{}\n", message);
  io::stdout().flush().unwrap();
}

fn set_status_message(status_text: &str) {
  print!("{}{}\n", "STATUS: ", status_text);
  io::stdout().flush().unwrap();
}

fn get_user_input(message: &str) -> String {
  let mut line = String::new();
  print!("{}", message);
  io::stdout().flush().unwrap();
  std::io::stdin().read_line(&mut line).unwrap();
  line
}

#[tokio::main]
async fn main() {
  let mut line = String::new();
  let mut world = String::new();
  let mut place = String::new();
  let mut npc = String::new();

  println!("Welcome to Simulacra. Type 'new' to generate a new world and simulation. Enter 'exit' to quit");
  loop{
    let line = get_user_input("Command: ");
    match line.trim() {
      "help" => {
        set_status_message("Help");
        print_console("Commands:");
        print_console("new - Generate a new world, place and npc");
        print_console("exit - Exit the program");
      },
      "new" => {
        match npc.is_empty() {
          true => {
            set_status_message("Generating new world, place and npc");
            (world, place, npc) = get_new_world_place_npc().await;
            set_status_message("World, place and npc generated. Use 'world', 'place', or 'npc'commands to view details.");
          },
          false => {
            print_console("A world, place and npc have already been generated. Use 'clear' command to discard before generating a new simulation.");
          }
        }
      },
      "new-world" => {
        match world.is_empty() {
          true => {
            set_status_message("Generating new world.");
            world = get_new_world().await;
            set_status_message("World generated. Use 'world' to view details.");
          },
          false => {
            print_console("A world has already been generated. Use 'clear' command to discard before generating a new simulation.");
          }
        }
      },      
      "clear" => {
        set_status_message("Clearing world, place and npc");
        world = String::new();
        place = String::new();
        npc = String::new();
      },
      "world" => {
        match world.is_empty() {
          true => {
            print_console("No world generated. Use 'new' command to generate a new world, place and npc.");
          },
          false => {
            let msg = format!("WORLD: \n{}\n", &world);
            print_console(&msg);
          }
        }
      },
      "place" => {
        match place.is_empty() {
          true => {
            print_console("No place generated. Use 'new' command to generate a new world, place and npc.");
          },
          false => {
            let msg = format!("PLACE: \n{}\n", &place);
            print_console(&msg);
          }
        }
      },
      "npc" => {
        match npc.is_empty() {
          true => {
            print_console("No npc generated. Use 'new' command to generate a new world, place and npc.");
          },
          false => {
            let msg = format!("NPC: \n{}\n", &npc);
            print_console(&msg);
          }
        }
      },
      "exit" => {
        set_status_message("Exiting");
        break;
      },
      _ => {
        set_status_message("Invalid command");
      }
    }
  }
}