// pub mod datastore;
// pub mod cognition;

// use datastore::datastore::*;
// use rusqlite::{Result};

// fn main() -> Result<()> {

//     // Insert some sample data into each table
//     let world_id = insert_into_world("World 1", "A world of magic and mystery.", "2023-04-21")?;
//     let npc_id = insert_into_npc(world_id, "Bob", "A friendly local shopkeeper.", "2023-04-21")?;
//     insert_into_memory(npc_id, 1, "Bob sold me a potion yesterday.", "2023-04-22")?;

//     // get and print data from the world table
//     if let Some(world) = get_world_by_id(1)? {
//         println!("World {}: {} ({})", world.id, world.name, world.date);
//         println!("Description: {}", world.description);
//     }

//     // get and print data from the npc table
//     if let Some(npc) = get_npc_by_id(1)? {
//         println!("NPC {}: {} ({})", npc.id, npc.name, npc.date);
//         println!("Description: {}", npc.description);
//         println!("World ID: {}", npc.world_id);
//     }

//     // get and print data from the memory table
//     if let Some(memory) = get_memory_by_id(1)? {
//         println!("Memory {}: {} ({})", memory.id, memory.memory, memory.date);
//         println!("Type ID: {}", memory.type_id);
//         println!("NPC ID: {}", memory.npc_id);
//     }

//     // Get memories for NPC with ID 1
//     let memories = get_memories_by_npc_id(1, None).unwrap();
//     println!("Memories for NPC 1: {:?}", memories);

//     // Get the 10 most recent memories for NPC with ID 2
//     let memories = get_memories_by_npc_id(2, Some(10)).unwrap();
//     println!("Most recent 10 memories for NPC 2: {:?}", memories);

//     Ok(())
// }