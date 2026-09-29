CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('todo', 'in_progress', 'done')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    position INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX tasks_status_position ON tasks (status, position);
