#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Option<i64>,
    pub inventory_id: i64,
    pub kind: TransactionKind,
    pub quantity: i64,
    pub unit_price_cents: i64,
}

#[derive(Debug, Clone)]
pub enum TransactionKind {
    Buy,
    Sell,
}

impl TransactionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}
