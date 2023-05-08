use rusqlite::{Result, Connection, Row, params};

pub static DB_NAME: &str = "simulacra.db";
    
fn get_db_conn() -> Result<Connection> {
    let conn = Connection::open(DB_NAME)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS simulation (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                name            TEXT NOT NULL,
                date            TEXT NOT NULL,
                cycles          INTEGER NOT NULL
                )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS world (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                simulation_id   INTEGER NOT NULL REFERENCES simulation (id),
                name            TEXT NOT NULL,
                summary         TEXT NOT NULL,
                description     TEXT NOT NULL,
                date            TEXT NOT NULL
                )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS place (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                world_id        INTEGER NOT NULL REFERENCES world (id),
                name            TEXT NOT NULL,
                summary         TEXT NOT NULL,
                description     TEXT NOT NULL,
                date            TEXT NOT NULL
                )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS npc (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                world_id        INTEGER NOT NULL REFERENCES world (id),
                name            TEXT NOT NULL,
                summary         TEXT NOT NULL,
                description     TEXT NOT NULL,
                date            TEXT NOT NULL
                )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS world_memory (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                world_id        INTEGER NOT NULL REFERENCES world (id),
                type_id         INTEGER NOT NULL,
                memory          TEXT NOT NULL,
                date            TEXT NOT NULL
                )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS place_memory (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                place_id        INTEGER NOT NULL REFERENCES place (id),
                type_id         INTEGER NOT NULL,
                memory          TEXT NOT NULL,
                date            TEXT NOT NULL
                )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS npc_memory (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                npc_id          INTEGER NOT NULL REFERENCES npc (id),
                type_id         INTEGER NOT NULL,
                memory          TEXT NOT NULL,
                date            TEXT NOT NULL
                )",
        [],
    )?;              
    Ok(conn)
}


pub fn save_simulation(name: &str) -> Result<i32> {
    let conn = get_db_conn().unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let query = format!("INSERT INTO simulation (name, date, cycles) VALUES ('{}', '{}', 0)", name, &date);
    conn.execute(
        &query,
        [],
    );
    let id = conn.last_insert_rowid() as i32;
    Ok(id)
}

pub fn increment_simuation_cycles(simulation_id: i32) {
    let conn = get_db_conn().unwrap();
    let query = format!("UPDATE simulation SET cycles = cycles + 1 WHERE id = {}", simulation_id);
    conn.execute(
        &query,
        [],
    );
}

pub fn save_world(simulation_id: i32, name: String, summary: String, description: String) -> Result<i32> {
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let conn = get_db_conn().unwrap();
    conn.execute(
        "INSERT INTO world (simulation_id, name, summary, description, date) VALUES (?, ?, ?, ?, ?)",
        params![simulation_id, &name, &summary, &description, &date],
    )?;   
    let id = conn.last_insert_rowid() as i32;
    Ok(id)
}

pub fn save_place(world_id: i32, name: String, summary: String, description: String) -> Result<i32> {
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let conn = get_db_conn().unwrap();
    conn.execute(
        "INSERT INTO place (world_id, name, summary, description, date) VALUES (?, ?, ?, ?, ?)",
        params![world_id, &name, &summary, &description, &date],
    )?;    
    let id = conn.last_insert_rowid() as i32;
    Ok(id)
}

pub fn save_npc(world_id: i32, name: String, summary: String, description: String) -> Result<i32> {
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let conn = get_db_conn().unwrap();
    conn.execute(
        "INSERT INTO npc (world_id, name, summary, description, date) VALUES (?, ?, ?, ?, ?)",
        params![world_id, &name, &summary, &description, &date],
    )?;     
    let id = conn.last_insert_rowid() as i32;
    Ok(id)
}

pub fn save_world_memory(world_id: i32, type_id: i32, memory: String) -> Result<i32> {
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let conn = get_db_conn().unwrap();
    conn.execute(
        "INSERT INTO world_memory (world_id, type_id, memory, date) VALUES (?, ?, ?, ?, ?)",
        params![world_id, type_id, &memory, &date],
    )?;     
    let id = conn.last_insert_rowid() as i32;
    Ok(id)
}

pub fn save_place_memory(place_id: i32, type_id: i32, memory: String) -> Result<i32> {
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let conn = get_db_conn().unwrap();
    conn.execute(
        "INSERT INTO place_memory (place_id, type_id, memory, date) VALUES (?, ?, ?, ?, ?)",
        params![place_id, type_id, &memory, &date],
    )?; 
    let id = conn.last_insert_rowid() as i32;
    Ok(id)
}

pub fn save_npc_memory(npc_id: i32, type_id: i32, memory: String) -> Result<i32> {
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let conn = get_db_conn().unwrap();
    conn.execute(
        "INSERT INTO place_memory (npc_id, type_id, memory, date) VALUES (?, ?, ?, ?, ?)",
        params![npc_id, type_id, &memory, &date],
    )?;     
    let id = conn.last_insert_rowid() as i32;
    Ok(id)
}

#[derive(Debug)]
pub struct World {
    pub id: i32,
    pub simulation_id: i32,
    pub name: String,
    pub summary: String,
    pub description: String,
    pub date: String,
}

pub fn get_world_by_id(id: i32) -> Result<Option<World>> {
    let conn = get_db_conn().unwrap();
    let mut stmt = conn.prepare("SELECT id, simulation_id, name, summary,description, date FROM world WHERE id = ?1")?;
    let mut rows = stmt.query(&[&id])?;
    if let Some(row) = rows.next()? {
        let world = World {
            id: row.get(0)?,
            simulation_id: row.get(1)?,
            name: row.get(2)?,
            summary: row.get(3)?,
            description: row.get(4)?,
            date: row.get(5)?,
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
    pub summary: String,
    pub detail: String,
    pub date: String,
}

pub fn get_npc_by_id(id: i32) -> Result<Option<Npc>> {
    let conn = get_db_conn().unwrap();
    let mut stmt = conn.prepare("SELECT id, world_id, name, summary, detail, date FROM npc WHERE id = ?1")?;
    let mut rows = stmt.query(&[&id])?;
    if let Some(row) = rows.next()? {
        let npc = Npc {
            id: row.get(0)?,
            world_id: row.get(1)?,
            name: row.get(2)?,
            summary: row.get(3)?,
            detail: row.get(3)?,
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

#[derive(Debug)]
pub struct Simulation {
    pub id: i32,
    pub name: String,
    pub date: String,
    pub cycles: i32,
}

pub fn get_simulations() -> Result<Vec<Simulation>> {
    let conn = get_db_conn().unwrap();
    let mut stmt = conn.prepare("SELECT id, name, date, cycles FROM simulation ORDER BY cycles desc")?;

    let mut rows = stmt.query([])?;

    let mut simulations = Vec::new();
    while let Some(row) = rows.next()? {
        let simulation = Simulation {
            id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
            cycles: row.get(3)?,
        };
        simulations.push(simulation);
    }
    Ok(simulations)
}

#[derive(Debug)]
pub struct SimulationSummary {
    pub id: i32,
    pub world_name: String,
    pub world_summary: String,
    pub date: String,
    pub cycles: i32,
    pub place_name: String,
    pub place_summary: String,
    pub npc_name: String,
    pub npc_summary: String,        
}

pub fn get_simulation_list() -> Result<Vec<SimulationSummary>> {
    let conn = get_db_conn().unwrap();

    let mut stmt = conn.prepare(
        "SELECT s.id [simulation_id], w.name [world_name], w.summary [world_summary], s.date, s.cycles, 
        p.name [place_name], p.summary [place_summary], n.name [npc_name], n.summary [npc_summary]
        FROM simulation as s
        JOIN world as w on s.id = w.simulation_id
        JOIN place as p on p.world_id = w.id
        JOIN npc as n on n.world_id = w.id
        ORDER BY s.date desc",
    )?;

    let simulation_iter = stmt.query_map([], |row| {
        Ok(SimulationSummary {
            id: row.get(0)?,
            world_name: row.get(1)?,
            world_summary: row.get(2)?,
            date: row.get(3)?,
            cycles: row.get(4)?,
            place_name: row.get(5)?,
            place_summary: row.get(6)?,
            npc_name: row.get(7)?,
            npc_summary: row.get(8)?,
        })
    })?;

    let mut simulations = Vec::new();
    for simulation in simulation_iter {
        simulations.push(simulation?);
    }

    Ok(simulations)
}