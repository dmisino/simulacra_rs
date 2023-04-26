use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;

pub enum AgentMessage {
    Message(String),
}
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
        NpcAgent { id, name, summary, description, current_state, interval }
    }

    pub fn start(&mut self, sender: Sender<AgentMessage>, receiver: Receiver<AgentMessage>) {
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

        thread::spawn(move || {

            loop {
                match receiver.try_recv() {
                    Ok(AgentMessage::Message(message)) => {
                        println!("{} received message: {}", agent.name, message);
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
                        sender.send(AgentMessage::Message(message)).unwrap();
                    },
                    1 => {
                        let message = format!("{} (id {}) is running", name, id);
                        sender.send(AgentMessage::Message(message)).unwrap();
                    },
                    2 => {
                        let message = format!("{} (id {}) is in interactive mode", name, id);
                        sender.send(AgentMessage::Message(message)).unwrap();
                    },
                    3 => {
                        let message = format!("{} (id {}) is exiting", name, id);
                        sender.send(AgentMessage::Message(message)).unwrap();
                        break;
                    },
                    _ => break,
                }
                thread::sleep(interval);
            }
            drop(agent);
        });
    }

    pub fn set_state(&mut self, state: u32) {
        self.current_state = state;
    }
}
