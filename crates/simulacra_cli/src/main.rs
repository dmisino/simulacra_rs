//use std::sync::mpsc::{Sender, Receiver};
//use std::time::Duration;
//use simulacra::agent::{ NpcAgent };
//use std::thread;
use simulacra_lib::*;
use tokio;

#[tokio::main]
async fn main() {
    // Start the simulacra framework
    simulacra_lib::start("main.yaml".to_string()).await;


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

}