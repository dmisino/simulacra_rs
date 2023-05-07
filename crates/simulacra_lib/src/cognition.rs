/*
Saving memories

Agents perform whatever actions or interactions are a part of their programming. This is specific to each implementation. Given some interaction or event, the text of this event (presumably generated by a language model) is fed to a languge model which is tasked with extracting a concise list of memories from the overall text. These are de-duped and saved as memories.

Retrieving memories

When prompting a language model for an agent action at each time interval, the memories most relevant to the current situation need to be included in the prompt. Surfacing relevant memories is a significant challenge. One method invloves ranking memories based on:
  
  1. Recency - An exponential decay function is used to reduce the relevance of older memories.
  2. Relevance - A language model prompt describes the current situation, and asks for a ranking from 1 to 10 of how relevant each memory is to the current situation. 
  3. Importance - This refers to how important the memory is to the individual agent. A language model prompt describes the agent, its purpose, personality, memories and so on as is available, and then asks for a ranking from 1 to 10 of how important each memory is likely to be to that individual.

  The results of these three rankings are combined to produce a final ranking for each memory. The top memories that fit within the prompt context are then passed to the language model to be included in the  prompt to generate an agent action or response.

Reflections

Reflections are new thoughts formed from existing memories. On occasion the system will pull available memories, and send them to a language model with instructions to "reflect" on those memories and draw new thoughts or conclusions, which are then saved as new memories so they can help influence future behaviors. This allows for more organic and natural behavior from agents.


*/