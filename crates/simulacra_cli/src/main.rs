use std::sync::mpsc;
use std::time::Duration;
use simulacra::agent::{ NpcAgent, AgentMessage };
use std::thread;

fn main() {
    // Create a channel for sending and receiving messages
    let (caller_to_agent_sender, caller_to_agent_receiver) = mpsc::channel::<AgentMessage>();
    let (agent_to_caller_sender, agent_to_caller_receiver) = mpsc::channel::<AgentMessage>();

    // Instantiate a new NPC agent
    let mut agent = NpcAgent::new(
        1,
        "Bob".to_string(),
        "Bob is an AI created from the consciousness of a dead guy who was named Bob".to_string(),
        "Bob was a computer programmer whose consciousness was uploaded to a computer just before he was killed in a car accident. He then awoke as an AI 80 years later.".to_string(),
        1,
        Duration::from_secs(5),
    );

    // Start the agent in a separate thread
    agent.start(agent_to_caller_sender.clone(), caller_to_agent_receiver);

    // Receive and print messages from agents
    let mut n: u32 = 0;
    loop {
        n = n + 1;
        match agent_to_caller_receiver.try_recv() {
            Ok(AgentMessage::Message(message)) => println!("Received message from agent: {}", message),
            Err(_) => {},
        }
        if n == 10 {
          // Send a message to the agent to modify its state;
          caller_to_agent_sender.clone().send(AgentMessage::Message("set_state:0".to_string())).unwrap();
        }
        if n == 20 {
          // Send a message to the agent to modify its state;
          caller_to_agent_sender.clone().send(AgentMessage::Message("set_state:1".to_string())).unwrap();
        }
        thread::sleep(Duration::from_secs(1));
    }

}
