# simulacra
simulacra is a simulation engine and testbed for creating intelligent, interactive agents inside evolving narrative worlds using large language models.

Large Language models (LLMs) act as stateless computation engines, where their world view is generated instantly with each prompt, based on training data and the context that is provided to it. Language models can be trained with new or domain specific data, allowing them to respond in a useful way to specific types of questions. But beyond this, with no memory of their own, the key to harnessing natural language processing for creating systems that provide immersive, continuous experiences is in building supporting frameworks that provide context and instruction. Prompts are structured to provide everything the language model requires to rebuild a world view so it can generate output that appears intelligent and believable.

This project attempts to use LLM technology to create agents that model realistic behavior, that a user can interact with, and whose world and story will continue to evolve over time. The main purpose of this effort is to experiment with, and demonstrate, what is currently possible using language models for creating believable, interactive characters and dynamic story lines.

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

Agents perform actions or have interactions as part of their function. In this particular project, these actions arise from a language model being prompted to generate the agents next action, given a characters background, memories, thoughts, their world and their current surroundings. The text describing each event is again fed to a languge model, which is tasked with extracting a concise list of memories. These are then saved as individual time-stamped memories.

### Retrieving memories

When prompting a language model for an agent action at each time interval, the memories most relevant to the current situation need to be included. Surfacing the most relevant memories is a challenge, and one of the more important areas of research in developing agents based on language models. One method invloves ranking memories based on:
  
  1. Recency - An exponential decay function is used to reduce the relevance of older memories.
  2. Relevance - A language model prompt describes the current situation, and asks for a ranking from 1 to 10 of how relevant each memory is to the current situation. 
  3. Importance - This refers to how important the memory is to the individual agent. A language model prompt describes the agent, its purpose, personality, memories and so on as is available, and then asks for a ranking from 1 to 10 of how important each memory is likely to be to that individual.

The results of these three rankings are combined to produce a final ranking score for each memory. The top memories that fit within the prompt context are then passed to the language model to be factored into deciding how an agent will react.

### Reflections

Reflections are new thoughts formed from existing memories. On occasion the system will pull available memories, and send them to a language model with instructions to "reflect" on those memories and draw conclusions or form new thoughts. These are then saved as new agent memories. This allows for more organic and natural behavior from agents.

### World and environment

A basic overview of the world and its major events (world memories) are added to each agent action or conversational prompt. World memories may also be added manually by the user.

A description of the local area, and an environment tree are also added to each agent prompt, giving the agent the opportunity to react to, or interact with, their environment.

### Events

On occasion, random events may be created, getting injected into world, location or agent memories as appropriate. These will be incorporated into the agents memory stream, impacting their emerging story, actions and dialog. 

### Letting the simulation run

TBD

## Commands

TBD

## Acknowledgements

[rozgo](https://github.com/rozgo)

[Generative Agents: Interactive Simulacra of Human Behavior](https://arxiv.org/pdf/2304.03442.pdf)