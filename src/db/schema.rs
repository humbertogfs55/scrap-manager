use rusqlite::{Connection, Result};

pub fn create_all(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS inventory (
            id              INTEGER PRIMARY KEY,
            sku             TEXT NOT NULL UNIQUE,
            name            TEXT NOT NULL,   
            quantity        INTEGER NOT NULL DEFAULT 0,
            avg_cost        INTEGER NOT NULL,
            created_at      TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS transactions(
            id              INTEGER PRIMARY KEY,
            inventory_id    INTEGER NOT NULL,
            kind            TEXT NOT NULL CHECK(kind IN ('buy','sell')),
            quantity        INTEGER NOT NULL,
            unit_price      INTEGER NOT NULL,
            created_at      TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY(inventory_id) REFERENCES inventory(id) ON DELETE RESTRICT
        );

        CREATE INDEX IF NOT EXISTS idx_tx_inventory
            ON  transactions(inventory_id);
        "#,
    )
}
