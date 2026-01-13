use crate::domain::{inventory::Inventory, transaction::Transaction};
use rusqlite::{Connection, Result, params};

pub fn insert_inventory(conn: &Connection, inv: &Inventory) -> Result<()> {
    conn.execute(
        r#"INSERT INTO inventory (sku, name, quantity, avg_cost)
           VALUES (?1, ?2, ?3, ?4)"#,
        params![inv.sku, inv.name, inv.quantity, inv.avg_cost_cents],
    )?;
    Ok(())
}

pub fn list_inventory(conn: &Connection) -> Result<Vec<Inventory>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, sku, name, quantity, avg_cost
           FROM inventory
           ORDER BY name"#,
    )?;

    let rows = stmt.query_map([], |r| {
        Ok(Inventory {
            id: r.get(0)?,
            sku: r.get(1)?,
            name: r.get(2)?,
            quantity: r.get(3)?,
            avg_cost_cents: r.get(4)?,
        })
    })?;

    rows.collect()
}

pub fn insert_transaction(conn: &Connection, tx: &Transaction) -> Result<()> {
    conn.execute(
        r#"INSERT INTO transactions
           (inventory_id, kind, quantity, unit_price)
           VALUES (?1, ?2, ?3, ?4)"#,
        params![
            tx.inventory_id,
            tx.kind.as_str(),
            tx.quantity,
            tx.unit_price_cents
        ],
    )?;
    Ok(())
}
