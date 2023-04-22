use rusqlite::{types::Value, types::FromSql, Error, Result, Connection, Row};

fn main() -> Result<()> {
    let conn = Connection::open("memory.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS world (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                description     TEXT NOT NULL,
                date            TEXT NOT NULL
                )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS npc (
                id              INTEGER PRIMARY KEY,
                world_id        INTEGER NOT NULL REFERENCES world (id),
                name            TEXT NOT NULL,
                description     TEXT NOT NULL,
                date            TEXT NOT NULL
                )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS memory (
                id              INTEGER PRIMARY KEY,
                npc_id          INTEGER NOT NULL REFERENCES npc (id),
                type_id         INTEGER NOT NULL,
                memory          TEXT NOT NULL,
                date            TEXT NOT NULL
                )",
        [],
    )?;

    // Create functions to insert into each table

    fn insert_into_world(conn: &Connection, name: &str, description: &str, date: &str) -> Result<i32> {
        conn.execute(
            "INSERT INTO world (name, description, date) VALUES (?1, ?2, ?3)",
            &[name, description, date],
        )?;

        let id = conn.last_insert_rowid() as i32;

        Ok(id)
    }

    fn insert_into_npc(conn: &Connection, world_id: i32, name: &str, description: &str, date: &str) -> Result<i32> {
        let query = format!("INSERT INTO npc (world_id, name, description, date) VALUES ({}, '{}', '{}', {})", world_id, name, description, date);
        conn.execute(
            &query,
            []
        )?;

        let id = conn.last_insert_rowid() as i32;

        Ok(id)
    }

    fn insert_into_memory(conn: &Connection, npc_id: i32, type_id: i32, memory: &str, date: &str) -> Result<i32> {
        let query = format!("INSERT INTO memory (npc_id, type_id, memory, date) VALUES ({}, {}, '{}', {})", npc_id, type_id, memory, date);
        conn.execute(
            &query,
            [],
        )?;

        let id = conn.last_insert_rowid() as i32;

        Ok(id)
    }

    #[derive(Debug)]
    struct World {
        id: i32,
        name: String,
        description: String,
        date: String,
    }

    fn get_world_by_id(conn: &Connection, id: i32) -> Result<Option<World>> {
        let mut stmt = conn.prepare("SELECT id, name, description, date FROM world WHERE id = ?1")?;
        let mut rows = stmt.query(&[&id])?;
        if let Some(row) = rows.next()? {
            let world = World {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                date: row.get(3)?,
            };
            Ok(Some(world))
        } else {
            Ok(None)
        }
    }

    #[derive(Debug)]
    struct Npc {
        id: i32,
        world_id: i32,
        name: String,
        description: String,
        date: String,
    }
    
    fn get_npc_by_id(conn: &Connection, id: i32) -> Result<Option<Npc>> {
        let mut stmt = conn.prepare("SELECT id, world_id, name, description, date FROM npc WHERE id = ?1")?;
        let mut rows = stmt.query(&[&id])?;
        if let Some(row) = rows.next()? {
            let npc = Npc {
                id: row.get(0)?,
                world_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                date: row.get(4)?,
            };
            Ok(Some(npc))
        } else {
            Ok(None)
        }
    }

    #[derive(Debug)]
    struct Memory {
        id: i32,
        npc_id: i32,
        type_id: i32,
        memory: String,
        date: String,
    }
    impl From<Row<'_>> for Memory {
        fn from(row: Row<'_>) -> Self {
            let id = row.get(0).unwrap_or_default();
            let npc_id = row.get(1).unwrap_or_default();
            let type_id = row.get(2).unwrap_or_default();
            let memory = row.get(3).unwrap_or_default();
            let date = row.get(4).unwrap_or_default();
    
            Memory {
                id,
                npc_id,
                type_id,
                memory,
                date,
            }
        }
    }

    fn get_memory_by_id(conn: &Connection, id: i32) -> Result<Option<Memory>> {
        let mut stmt = conn.prepare("SELECT id, npc_id, type_id, memory, date FROM memory WHERE id = ?1")?;
        let mut rows = stmt.query(&[&id])?;
        if let Some(row) = rows.next()? {
            let memory = Memory {
                id: row.get(0)?,
                npc_id: row.get(1)?,
                type_id: row.get(2)?,
                memory: row.get(3)?,
                date: row.get(4)?,
            };
            Ok(Some(memory))
        } else {
            Ok(None)
        }
    }

    fn get_memories_by_npc_id(conn: &Connection, npc_id: i32, limit: Option<i32>) -> Result<Vec<Memory>> {
        let mut stmt = conn.prepare(
            match limit {
                Some(_) => "SELECT * FROM memory WHERE npc_id = ?1 ORDER BY date DESC LIMIT ?2",
                None => "SELECT * FROM memory WHERE npc_id = ?1 ORDER BY date DESC",
            }
        )?;
    
        let mut rows = match limit {
            Some(limit_value) => stmt.query([npc_id, limit_value])?,
            None => stmt.query([npc_id])?,
        };
    
        let mut memories = Vec::new();
        while let Some(row) = rows.next()? {
            let memory = Memory {
                id: row.get(0)?,
                npc_id: row.get(1)?,
                type_id: row.get(2)?,
                memory: row.get(3)?,
                date: row.get(4)?,
            };
            memories.push(memory);
        }
        Ok(memories)
    }

    // Insert some samepl data into each table
    let world_id = insert_into_world(&conn, "World 1", "A world of magic and mystery.", "2023-04-21")?;
    let npc_id = insert_into_npc(&conn, world_id, "Bob", "A friendly local shopkeeper.", "2023-04-21")?;
    insert_into_memory(&conn, npc_id, 1, "Bob sold me a potion yesterday.", "2023-04-22")?;

    // get and print data from the world table
    if let Some(world) = get_world_by_id(&conn, 1)? {
        println!("World {}: {} ({})", world.id, world.name, world.date);
        println!("Description: {}", world.description);
    }

    // get and print data from the npc table
    if let Some(npc) = get_npc_by_id(&conn, 1)? {
        println!("NPC {}: {} ({})", npc.id, npc.name, npc.date);
        println!("Description: {}", npc.description);
        println!("World ID: {}", npc.world_id);
    }

    // get and print data from the memory table
    if let Some(memory) = get_memory_by_id(&conn, 1)? {
        println!("Memory {}: {} ({})", memory.id, memory.memory, memory.date);
        println!("Type ID: {}", memory.type_id);
        println!("NPC ID: {}", memory.npc_id);
    }

    // Get memories for NPC with ID 1
    let memories = get_memories_by_npc_id(&conn, 1, None).unwrap();
    println!("Memories for NPC 1: {:?}", memories);

    // Get the 10 most recent memories for NPC with ID 2
    let memories = get_memories_by_npc_id(&conn, 2, Some(10)).unwrap();
    println!("Most recent 10 memories for NPC 2: {:?}", memories);

    Ok(())
}