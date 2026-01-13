#[derive(Debug, Clone)]
pub struct Inventory {
    pub id: Option<i64>,
    pub sku: String,
    pub name: String,
    pub quantity: i64,
    pub avg_cost_cents: i64,
}
