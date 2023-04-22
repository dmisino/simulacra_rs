# simulacra
simulacra is a framework for creating and testing agents simulating intelligent, interactive behavior using large language models.

Large Language models (LLMs) act as stateless natural language computation engines, where their world view is generated in an instant based on their training data and the context that is provided in each prompt. The key to harnessing their power for creating systems that provide immersive experiences useful for all types of interactive media is in creating the supporting frameworks that structure prompts to provide everything necessary for the model to understand and successfully synthesise an appropriate output that gives the allusion of continuity. 

To that end, the LLM must be supplied in every call with contextual queues, a memory of past interactions, and effective guidance in the processing of each query for it to successfully produce the desired results.

## Features

- Automatically generate agents with random personality traits and backstory
- Autmatically creates an environment tree that describes the world the characters inhabit
- Allow the agents to freely go about their lives, interact with each other, develop friendships, adversaries, or anything else that normally happens in the life of a person

## Commands

After building and running, an environment (stored in memory as an environment tree) will have been created. You can then add people to the environment and they will automatically go about their lives. A running history of activities and interactions will be displayed as they occur. At any time you can see what is happening with your people, or modify some aspect of the simulation using available commands. 

## Installation

To run this project, you'll need Rust installed on your machine. You can follow the instructions at https://www.rust-lang.org/tools/install.

For a data storage, this project uses [RocksDB](https://rocksdb.org/), a high performance, embeddable persistent key-value store. This requires [LLVM compiler](https://github.com/llvm/llvm-project) be installed on your system. You can find the latest installer for LLVM appropriate for your os [here](https://github.com/llvm/llvm-project/releases/). 

## Clone the repository
```
git clone https://github.com/dmisino/simulacra.git
cd asimulacra
```

### Configure API keys

This project currently uses OpenAI's GPT-3.5-Turbo language model. I will add options to use a locally running language model such as LLaMa (which you can grab from my [docker account](https://hub.docker.com/r/dmisino/dalai)), but for now you'll need an OpenAI API account. To set environment variables use the following:
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

## Usage

Once you've built and run ai_simulacra, you'll be prompted with something (TODO). To exit the system, type "exit".

## Acknowledgements

Special thanks to [rozgo](https://github.com/rozgo)

Other inspiration for this project:

[Generative Agents: Interactive Simulacra of Human Behavior](https://arxiv.org/pdf/2304.03442.pdf)

[ReAct: Synergizing Reasoning and Acting in Language Models](https://arxiv.org/pdf/2210.03629.pdf)
