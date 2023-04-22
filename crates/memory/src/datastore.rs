pub mod datastore {
    use rusqlite::{Result, Connection, Row};

    pub static DB_NAME: &str = "memory.db";
        
    fn get_db_conn() -> Result<Connection> {
        let conn = Connection::open(DB_NAME)?;
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
        Ok(conn)
    }

    pub fn insert_into_world(name: &str, description: &str, date: &str) -> Result<i32> {
        let conn = get_db_conn()?;
        conn.execute(
            "INSERT INTO world (name, description, date) VALUES (?1, ?2, ?3)",
            &[name, description, date],
        )?;

        let id = conn.last_insert_rowid() as i32;

        Ok(id)
    }

    pub fn insert_into_npc(world_id: i32, name: &str, description: &str, date: &str) -> Result<i32> {
        let conn = get_db_conn()?;
        let query = format!("INSERT INTO npc (world_id, name, description, date) VALUES ({}, '{}', '{}', {})", world_id, name, description, date);
        conn.execute(
            &query,
            []
        )?;

        let id = conn.last_insert_rowid() as i32;

        Ok(id)
    }

    pub fn insert_into_memory(npc_id: i32, type_id: i32, memory: &str, date: &str) -> Result<i32> {
        let conn = get_db_conn()?;
        let query = format!("INSERT INTO memory (npc_id, type_id, memory, date) VALUES ({}, {}, '{}', {})", npc_id, type_id, memory, date);
        conn.execute(
            &query,
            [],
        )?;

        let id = conn.last_insert_rowid() as i32;

        Ok(id)
    }

    #[derive(Debug)]
    pub struct World {
        pub id: i32,
        pub name: String,
        pub description: String,
        pub date: String,
    }

    pub fn get_world_by_id(id: i32) -> Result<Option<World>> {
        let conn = get_db_conn()?;
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
    pub struct Npc {
        pub id: i32,
        pub world_id: i32,
        pub name: String,
        pub description: String,
        pub date: String,
    }
    
    pub fn get_npc_by_id(id: i32) -> Result<Option<Npc>> {
        let conn = get_db_conn().unwrap();
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
    pub struct Memory {
        pub id: i32,
        pub npc_id: i32,
        pub type_id: i32,
        pub memory: String,
        pub date: String,
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

    pub fn get_memory_by_id(id: i32) -> Result<Option<Memory>> {
        let conn = get_db_conn().unwrap();
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

    pub fn get_memories_by_npc_id(npc_id: i32, limit: Option<i32>) -> Result<Vec<Memory>> {
        let conn = get_db_conn().unwrap();
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
}