use std::path::Path;
use std::sync::Mutex;

use rusqlite::{params, Connection, OptionalExtension};

const MIGRATIONS: &[(i64, &str, &str)] = &[(
    1,
    "create_tasks",
    include_str!("../migrations/001_create_tasks.sql"),
)];

pub struct DbState(pub Mutex<Connection>);

#[derive(Debug, Clone, serde::Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub position: i64,
}

pub fn open_file(path: &Path) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    migrate(&conn)?;
    Ok(conn)
}

pub fn migrate(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            applied_at TEXT NOT NULL
        );",
    )?;

    for (version, description, sql) in MIGRATIONS {
        let already: Option<i64> = conn
            .query_row(
                "SELECT version FROM schema_migrations WHERE version = ?1",
                [version],
                |row| row.get(0),
            )
            .optional()?;
        if already.is_some() {
            continue;
        }

        let tx = conn.unchecked_transaction()?;
        tx.execute_batch(sql)?;
        tx.execute(
            "INSERT INTO schema_migrations (version, description, applied_at)
             VALUES (?1, ?2, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))",
            params![version, description],
        )?;
        tx.commit()?;
    }

    Ok(())
}

fn with_db<T>(
    state: &DbState,
    f: impl FnOnce(&Connection) -> Result<T, String>,
) -> Result<T, String> {
    let conn = state.0.lock().map_err(|err| err.to_string())?;
    f(&conn)
}

fn map_err(err: rusqlite::Error) -> String {
    err.to_string()
}

fn require_title(title: &str) -> Result<&str, String> {
    let title = title.trim();
    if title.is_empty() {
        Err("Title is required".into())
    } else {
        Ok(title)
    }
}

fn require_status(status: &str) -> Result<&str, String> {
    match status {
        "todo" | "in_progress" | "done" => Ok(status),
        _ => Err("Status must be todo, in_progress, or done".into()),
    }
}

fn read_task(row: &rusqlite::Row<'_>) -> rusqlite::Result<Task> {
    Ok(Task {
        id: row.get(0)?,
        title: row.get(1)?,
        status: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
        position: row.get(5)?,
    })
}

const TASK_COLUMNS: &str = "id, title, status, created_at, updated_at, position";

fn list(conn: &Connection) -> Result<Vec<Task>, String> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {TASK_COLUMNS}
             FROM tasks
             ORDER BY
               CASE status
                 WHEN 'todo' THEN 0
                 WHEN 'in_progress' THEN 1
                 ELSE 2
               END,
               position ASC,
               id ASC"
        ))
        .map_err(map_err)?;
    let tasks = stmt
        .query_map([], read_task)
        .map_err(map_err)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(map_err)?;
    Ok(tasks)
}

fn create(conn: &Connection, title: &str) -> Result<Task, String> {
    let title = require_title(title)?;
    conn.query_row(
        &format!(
            "INSERT INTO tasks (title, status, created_at, updated_at, position)
             VALUES (
               ?1,
               'todo',
               strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
               strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
               COALESCE((SELECT MAX(position) + 1 FROM tasks WHERE status = 'todo'), 0)
             )
             RETURNING {TASK_COLUMNS}"
        ),
        params![title],
        read_task,
    )
    .map_err(map_err)
}

fn update_title(conn: &Connection, id: i64, title: &str) -> Result<Task, String> {
    let title = require_title(title)?;
    conn.query_row(
        &format!(
            "UPDATE tasks
             SET title = ?1,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE id = ?2
             RETURNING {TASK_COLUMNS}"
        ),
        params![title, id],
        read_task,
    )
    .map_err(|err| match err {
        rusqlite::Error::QueryReturnedNoRows => "Task not found".into(),
        other => map_err(other),
    })
}

fn set_status(conn: &Connection, id: i64, status: &str) -> Result<Task, String> {
    let status = require_status(status)?;
    conn.query_row(
        &format!(
            "UPDATE tasks
             SET status = ?1,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                 position = COALESCE(
                   (SELECT MAX(position) + 1 FROM tasks WHERE status = ?1 AND id != ?2),
                   0
                 )
             WHERE id = ?2
             RETURNING {TASK_COLUMNS}"
        ),
        params![status, id],
        read_task,
    )
    .map_err(|err| match err {
        rusqlite::Error::QueryReturnedNoRows => "Task not found".into(),
        other => map_err(other),
    })
}

fn delete(conn: &Connection, id: i64) -> Result<(), String> {
    let changed = conn
        .execute("DELETE FROM tasks WHERE id = ?1", params![id])
        .map_err(map_err)?;
    if changed == 0 {
        Err("Task not found".into())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn list_tasks(state: tauri::State<DbState>) -> Result<Vec<Task>, String> {
    with_db(&state, list)
}

#[tauri::command]
pub fn create_task(state: tauri::State<DbState>, title: String) -> Result<Task, String> {
    with_db(&state, |conn| create(conn, &title))
}

#[tauri::command]
pub fn update_task_title(
    state: tauri::State<DbState>,
    id: i64,
    title: String,
) -> Result<Task, String> {
    with_db(&state, |conn| update_title(conn, id, &title))
}

#[tauri::command]
pub fn set_task_status(
    state: tauri::State<DbState>,
    id: i64,
    status: String,
) -> Result<Task, String> {
    with_db(&state, |conn| set_status(conn, id, &status))
}

#[tauri::command]
pub fn delete_task(state: tauri::State<DbState>, id: i64) -> Result<(), String> {
    with_db(&state, |conn| delete(conn, id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_are_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        migrate(&conn).unwrap();

        let versions: i64 = conn
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| row.get(0))
            .unwrap();
        assert_eq!(versions, 1);

        let columns: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('tasks')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(columns, 6);
    }

    #[test]
    fn task_round_trip() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();

        let created = create(&conn, "  Ship the board  ").unwrap();
        assert_eq!(created.title, "Ship the board");
        assert_eq!(created.status, "todo");
        assert!(!created.created_at.is_empty());
        assert_eq!(created.created_at, created.updated_at);

        let renamed = update_title(&conn, created.id, "Ship the board today").unwrap();
        assert_eq!(renamed.title, "Ship the board today");

        let started = set_status(&conn, created.id, "in_progress").unwrap();
        assert_eq!(started.status, "in_progress");

        let listed = list(&conn).unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].status, "in_progress");

        delete(&conn, created.id).unwrap();
        assert!(list(&conn).unwrap().is_empty());
        assert_eq!(create(&conn, "   ").unwrap_err(), "Title is required");
        assert!(set_status(&conn, created.id, "later").is_err());
    }
}
