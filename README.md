# simulacra
simulacra is a simulation engine for creating intelligent, interactive agents inside of evolving narrative worlds using large language models. 

Large Language models (LLMs) act as stateless computation engines, where their world view is generated instantly each time it is called, based on training data and the context that is provided to it. Language models can be trained with new or domain specific data. But beyond this, with no memory of their own, the key to harnessing natural language processing for creating systems that provide immersive, ongoing experiences is in building the supporting frameworks that provide the needed context and instruction. Prompts are structured to provide everything the language model requires to understand and successfully synthesise an appropriate output, giving the illusion of intelligence and continuity of thought. 

This project attempts to use LLM technology to create agents that model realistic behavior, that a user can interact with, and whose world and story will continue to evolve over time. The main purpose of this effort is to experiment with, and demonstrate, what is currently possible using language models for creating believable, interactive characters and emergent story lines.

Note: This project is in development. Everything described here is subject to change.

## Features

- Generate agents with random personality traits, motivations and backstory
- Generate a world and specific location that the agent inhabits
- Agents maintain a running memory stream of past events and interactions, which influence future behaviors
- The world and its agents can continue to develop without user interaction
- Interact with an agent as a character from their story, with those interactions becoming part of their memory stream and influencing future behavior
- Interact with an agent as a user outside of their story, to question or test them without those interactions becoming part of their memories
- Manually inject world memories, location memories, or agent memories, to steer how the story unfolds 
- Inspect simulation internal details, such as world, location and agent summaries and memories

## Installation

To run this project, you'll need Rust installed on your machine. You can follow the instructions at https://www.rust-lang.org/tools/install.

## Clone the repository
```
git clone https://github.com/dmisino/simulacra.git
cd asimulacra
```

### Configure API keys

This project currently uses OpenAI's GPT-3.5-Turbo language model, so you will need an OpenAI API account. To set environment variables use the following:

```
# Windows
set OPENAI_API_KEY="your_openai_api_key"
# Linux
export OPENAI_API_KEY="your_openai_api_key"
```

### Build and run

```
cargo run --release -p simulacra_cli
```

## How does it work

### Defining a world, a location and an agent

TBD

### Saving memories

Agents perform whatever actions or interactions are a part of their programming. In this particular project, this is whatever results from language model prompted asking what the agent would do next, given background context about the character, its world and its current surroundings. Given some interaction or event, the text describing this event is fed to a languge model which is tasked with extracting a concise list of memories from the overall text. These are de-duped and saved as individual time-stamped memories.

### Retrieving memories

When prompting a language model for an agent action at each time interval, the memories most relevant to the current situation need to be included in the prompt. Surfacing relevant memories is a challenge, and one of the more important areas of research in developing agents based on language models. One method invloves ranking memories based on:
  
  1. Recency - An exponential decay function is used to reduce the relevance of older memories.
  2. Relevance - A language model prompt describes the current situation, and asks for a ranking from 1 to 10 of how relevant each memory is to the current situation. 
  3. Importance - This refers to how important the memory is to the individual agent. A language model prompt describes the agent, its purpose, personality, memories and so on as is available, and then asks for a ranking from 1 to 10 of how important each memory is likely to be to that individual.

The results of these three rankings are combined to produce a final ranking for each memory. The top memories that fit within the prompt context are then passed to the language model to be included in the  prompt to generate an agent action or response.

### Reflections

Reflections are new thoughts formed from existing memories. On occasion the system will pull available memories, and send them to a language model with instructions to "reflect" on those memories and draw new thoughts or conclusions, which are then saved as new agent memories. This allows for more organic and natural behavior from agents.

### World and environment

A basic overview of the world and its major events (world memories) are added to each agent prompt. At random intervals, new world events may be added which agents will become aware of, and factor into their actions. World memories may also be added manually by the user.

A description of the local area, and an environment tree are also added to each agent prompt, giving the agent the opportunity to interact with their environment, or make decisions based on environmental factors.

### Events

On occasion, random events may be created, getting injected into world, location or agent memories as appropriate. These will be incorporated into the agents emergent story, actions and dialog. 

### Letting the simulation run

TBD

## Commands

TBD

## Acknowledgements

[rozgo](https://github.com/rozgo)

[Generative Agents: Interactive Simulacra of Human Behavior](https://arxiv.org/pdf/2304.03442.pdf)