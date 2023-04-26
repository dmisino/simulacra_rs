use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
pub struct NpcAgent {
    id: u32,
    name: String,
    summary: String,
    description: String,
    current_state: u32,
    interval: Duration, 
}

impl NpcAgent {

    pub fn new(id: u32, name: String, summary: String, description: String, current_state: u32, interval: Duration) -> Self {
        NpcAgent { id, name, summary, description, current_state, interval}
    }

    pub fn start(&mut self) -> (Sender<String>, Receiver<String>) {
        let mut agent = NpcAgent {
            id: self.id,
            name: self.name.clone(),
            summary: self.summary.clone(),
            description: self.description.clone(),
            current_state: self.current_state,
            interval: self.interval,
        };
        let interval = self.interval;
        let id = self.id;
        let name = self.name.clone();
        agent.current_state = 1;

        // Create communication channels
        let (caller_to_agent_sender, caller_to_agent_receiver) = mpsc::channel::<String>();
        let (agent_to_caller_sender, agent_to_caller_receiver) = mpsc::channel::<String>();
    
        thread::spawn(move || {

            loop {
                match caller_to_agent_receiver.try_recv() {
                    Ok(message) => {
                        println!("{} (id {}) received message: {}", agent.name, agent.id, message);
                        let parts: Vec<&str> = message.split(':').collect();
                        let command = parts[0];
                        let value = parts[1];                        
                        match command {
                            "set_state" => {
                                agent.current_state = value.parse::<u32>().unwrap();
                            },
                            _ => {},
                        } 
                    }
                    Err(_) => {},
                }
                match agent.current_state {
                    0 => {
                        let message = format!("{} (id {}) is stopped", name, id);
                        agent_to_caller_sender.send(message).unwrap();
                    },
                    1 => {
                        let message = format!("{} (id {}) is running", name, id);
                        agent_to_caller_sender.send(message).unwrap();
                    },
                    2 => {
                        let message = format!("{} (id {}) is in interactive mode", name, id);
                        agent_to_caller_sender.send(message).unwrap();
                    },
                    3 => {
                        let message = format!("{} (id {}) is exiting", name, id);
                        agent_to_caller_sender.send(message).unwrap();
                        break;
                    },
                    _ => break,
                }
                thread::sleep(interval);
            }
            drop(agent);
        });
        (caller_to_agent_sender, agent_to_caller_receiver)
    }

    pub fn set_state(&mut self, state: u32) {
        self.current_state = state;
    }
}
