use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum OrderType {
    Market,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Order<Symbol> {
    pub order_id: u64,
    pub order_symbol: Symbol,
    pub price_symbol: Symbol,
    pub price: u64,
    pub quantity: u64,
    pub side: Side,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct OrderRequest<Symbol> {
    pub order: Order<Symbol>,
    pub order_type: OrderType,
}
