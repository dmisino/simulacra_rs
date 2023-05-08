//use std::sync::mpsc::{Sender, Receiver};
use std::thread::sleep;
use std::time::Duration;
//use simulacra::agent::{ NpcAgent };
//use std::thread;
use simulacra_lib::*;
use tokio;
use std::io::{self, Write};
use db::datastore::get_simulation_list;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};


#[tokio::main]
async fn main() {
    println!("Welcome to simulacra! Use 'help' for available commands");
    loop {
        print!("Command: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["new"] => {
                println!("Generating new simulation...");
                let simulation_id = simulacra_lib::start("fantasy_world".to_string()).await;
                println!("New simulation created with id {}", simulation_id);
            }
            ["list"] => {
                match get_simulation_list() {
                    Ok(simulations) => {
                        for simulation_summary in simulations.iter() {
                            println!(
                                "Simulation ID: {}\nWorld Name: {}\nWorld Summary: {}\nPlace Name: {}\nPlace Summary: {}\nNPC Name: {}\nNPC Summary: {}\nDate created: {}\nSimulation cycles run: {}\n",
                                simulation_summary.id,
                                simulation_summary.world_name,
                                simulation_summary.world_summary,
                                simulation_summary.place_name,
                                simulation_summary.place_summary,
                                simulation_summary.npc_name,
                                simulation_summary.npc_summary,                                                                
                                simulation_summary.date,
                                simulation_summary.cycles,
                            );
                        }
                    }
                    Err(error) => {
                        eprintln!("Error getting simulation list: {}", error);
                    }
                }                       
            }
            ["detail", id] => {
                println!("Not yet implemented");
            }
            ["run", id] => {
                println!("Running simulation with id {}. Press any key to stop.", id);
                loop {
                    println!("--agent activity--");
                    io::stdout().flush().expect("Failed to flush stdout");

                    sleep(Duration::from_secs(2));

                    if event::poll(Duration::from_millis(10)).unwrap() {
                        if let Event::Key(KeyEvent { code: KeyCode::Char(_), .. }) = event::read().unwrap() {
                            break;
                        }
                    }
                }
            }   
            ["talk", id] => {
                println!("Not yet implemented");
            }                                 
            ["help"] => {
                println!("If you are running this for the first time, run 'new' to generate a new simulation, then 'run' with the simulation id you just created.\n");
                println!("The following commands are available:");
                println!("new           Generate a new simulation, world, and agent");
                println!("list          List already created simulations");
                println!("detail <id>   Show full details of a simulation");
                println!("run <id>      Run a simulation by id");
                println!("talk <id>     Converse with the npc agent in a simulation");
                println!("help          Display help");
                println!("exit          Exit program");
            }
            ["exit"] | ["quit"] => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid command"),
        }
    }
}

    // // Track a list of agents to more easily manage message channels
    // let mut agent_list: Vec<(NpcAgent, Sender<String>, Receiver<String>)> = Vec::new();
    
    // // Instantiate NPC agent 1 ***************************
    // let mut agent1 = NpcAgent::new(
    //   1,
    //   "Bob".to_string(),
    //   "Bob is an AI created from the consciousness of a dead guy who was named Bob".to_string(),
    //   "Bob is a software engineer and entrepreneur who signs up for cryogenic freezing in the hopes of being revived in the future. However, he wakes up 117 years later to find that his body has been destroyed and his consciousness has been uploaded into a computer system.".to_string(),
    //   1,
    //   Duration::from_secs(5),
    // );   
    // let (agent1_sender, agent1_receiver) = agent1.start();
    // agent_list.push((agent1, agent1_sender.clone(), agent1_receiver));

    // // Instantiate NPC agent 2 ***************************
    // let mut agent2 = NpcAgent::new(
    //   2,
    //   "Jarvis".to_string(),
    //   "Jarvis is an AI created by Tony Stark (Iron Man)".to_string(),
    //   "Jarvis is an artificial intelligence created by Tony Stark to assist him in various tasks, such as managing his high-tech suit and providing strategic advice. Jarvis is known for his polite and dry sense of humor, as well as his unwavering loyalty to Stark.".to_string(),
    //   1,
    //   Duration::from_secs(8),
    // );
    // let (agent2_sender, agent2_receiver) = agent2.start();
    // agent_list.push((agent2, agent2_sender, agent2_receiver));

    // // Receive and print messages from all agents
    // let mut n: u32 = 0;
    // loop {
    //     n = n + 1;
    //     for (_, _, agent_to_caller_receiver) in &agent_list {
    //         match agent_to_caller_receiver.try_recv() {
    //             Ok(message) => println!("Received message: {}", message),
    //             Err(_) => {},
    //         }
    //     }
    //     // Test sending messages to agent 1 **********************
    //     if n == 10 { agent1_sender.clone().send("set_state:0".to_string()).unwrap(); }
    //     if n == 20 { agent1_sender.clone().send("set_state:1".to_string()).unwrap(); }

    //     thread::sleep(Duration::from_secs(1));
    // }