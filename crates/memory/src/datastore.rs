#[allow(dead_code)]
pub mod datastore {

    use rusqlite::{params, Connection, Result, Error, OpenFlags};
    use std::path::Path;
    
    const DB_NAME: &str = "C:\\_\\memory.db";

    #[derive(Debug)]
    pub struct World {
        id: i32,
        name: String,
        description: String,
        date: String,
    }

    #[derive(Debug)]
    pub struct Npc {
        id: i32,
        world_id: i32,
        name: String,
        description: String,
        date: String,
    }

    #[derive(Debug)]
    pub struct Memory {
        id: i32,
        npc_id: i32,
        type_id: i32,
        memory: String,
        date: String,
    }

    pub fn add_world(name: &str, description: &str, date: &str) -> Result<i32> {
        let conn = establish_connection().unwrap();
        conn.execute(
            "INSERT INTO world (name, description, date) VALUES (?1, ?2, ?3)",
            params![name, description, date],
        )?;
        Ok(conn.last_insert_rowid() as i32)
    }

    pub fn add_npc(world_id: i32, name: &str, description: &str, date: &str) -> Result<i32> {
        let conn = establish_connection().unwrap();
        conn.execute(
            "INSERT INTO npc (world_id, name, description, date) VALUES (?1, ?2, ?3, ?4)",
            params![world_id, name, description, date],
        )?;
        Ok(conn.last_insert_rowid() as i32)
    }

    pub fn add_memory(npc_id: i32, type_id: i32, memory: &str, date: &str) -> Result<i32> {
        let conn = establish_connection().unwrap();
        conn.execute(
            "INSERT INTO memory (npc_id, type_id, memory, date) VALUES (?1, ?2, ?3, ?4)",
            params![npc_id, type_id, memory, date],
        )?;
        Ok(conn.last_insert_rowid() as i32)
    }

    pub fn get_world(id: i32) -> Result<World> {
        let conn = establish_connection().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM world WHERE id = ?1")?;
        let world_iter = stmt.query_map(params![id], |row| {
            Ok(World {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                date: row.get(3)?,
            })
        })?;
        let world = world_iter
            .map(|w| w.unwrap())
            .next()
            .expect("Could not find world with that ID");
        Ok(world)
    }

    pub fn get_npc(id: i32) -> Result<Npc> {
        let conn = establish_connection().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM npc WHERE id = ?1")?;
        let npc_iter = stmt.query_map(params![id], |row| {
            Ok(Npc {
                id: row.get(0)?,
                world_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                date: row.get(4)?,
            })
        })?;
        let npc = npc_iter
            .map(|n| n.unwrap())
            .next()
            .expect("Could not find NPC with that ID");
        Ok(npc)
    }

    pub fn get_memory(id: i32) -> Result<Memory> {
        let conn = establish_connection().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM memory WHERE id = ?1")?;
        let memory_iter = stmt.query_map(params![id], |row| {
            Ok(Memory {
                id: row.get(0)?,
                npc_id: row.get(1)?,
                type_id: row.get(2)?,
                memory: row.get(3)?,
                date: row.get(4)?,
            })
        })?;
        let memory = memory_iter
            .map(|m| m.unwrap())
            .next()
            .expect("Could not find memory with that ID");
        Ok(memory)
    }

    pub fn select_memories(npc_id: i32, limit: Option<i32>) -> Result<Vec<Memory>, Error> {
        let conn = establish_connection().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM memory WHERE npc_id = ? ORDER BY date DESC LIMIT ?").unwrap();
        let limit = limit.unwrap_or(-1);
        let memory_iter = stmt.query_map(params![npc_id, limit], |row| {
            Ok(Memory {
                id: row.get(0)?,
                npc_id: row.get(1)?,
                type_id: row.get(2)?,
                memory: row.get(3)?,
                date: row.get(4)?,
            })
        })?;
        let memories: Result<Vec<Memory>, Error> = memory_iter.map(|m| m.map_err(|e| e.into())).collect();
        memories
    }

    pub fn create_tables(conn: &Connection) -> Result<(), Error> {
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

        Ok(())
    }

    pub fn establish_connection() -> Result<Connection, Error> {
        println!("establish_connection");
        let conn = if Path::new(DB_NAME).exists() {
            println!("establish_connection_2");
            Connection::open(DB_NAME)?
        } else {
            println!("establish_connection_3");
            let conn = Connection::open_with_flags(DB_NAME, OpenFlags::SQLITE_OPEN_CREATE)?;
         
            println!("establish_connection_4");
            create_tables(&conn)?;
            println!("establish_connection_5");
            conn
        };
        println!("establish_connection_6");
        Ok(conn)
    }
}