use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::time::SystemTime;

use crate::core::orders::{Order, OrderRequest, Side};

#[derive(Clone)]
struct OrderIndex {
    id: u64,
    price: u64,

    // @todo(vy): monotonic clock
    timestamp: SystemTime,
    order_side: Side,
}

impl Ord for OrderIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.price < other.price {
            match self.order_side {
                Side::Bid => Ordering::Less,
                Side::Ask => Ordering::Greater,
            }
        } else if self.price > other.price {
            match self.order_side {
                Side::Bid => Ordering::Greater,
                Side::Ask => Ordering::Less,
            }
        } else {
            other.timestamp.cmp(&self.timestamp)
        }
    }
}

impl PartialOrd for OrderIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OrderIndex {
    fn eq(&self, other: &Self) -> bool {
        if self.price > other.price || self.price < other.price {
            false
        } else {
            self.timestamp == other.timestamp
        }
    }
}

impl Eq for OrderIndex {}

pub struct OrderQueue<Symbol> {
    index_queue: BinaryHeap<OrderIndex>,
    orders: HashMap<u64, Order<Symbol>>,

    #[allow(dead_code)]
    order_side: Side,
}

impl<Symbol> OrderQueue<Symbol> {
    pub fn new(side: Side, capacity: usize) -> Self {
        OrderQueue {
            index_queue: BinaryHeap::with_capacity(capacity),
            orders: HashMap::with_capacity(capacity),
            order_side: side,
        }
    }

    pub fn peek(&mut self) -> Option<&Order<Symbol>> {
        let order_id = self.get_best_order_id()?;

        if self.orders.contains_key(&order_id) {
            self.orders.get(&order_id)
        } else {
            self.index_queue.pop()?;
            self.peek()
        }
    }

    pub fn insert(&mut self, order_request: OrderRequest<Symbol>) -> bool {
        if self.orders.contains_key(&order_request.order.id) {
            return false;
        }

        self.index_queue.push(OrderIndex {
            id: order_request.order.id,
            price: order_request.order.price,
            timestamp: order_request.timestamp,
            order_side: order_request.order.side,
        });

        self.orders
            .insert(order_request.order.id, order_request.order);

        true
    }

    fn get_best_order_id(&self) -> Option<u64> {
        let order_id = self.index_queue.peek()?;
        Some(order_id.id)
    }
}

pub struct OrderBook<Symbol> {
    #[allow(dead_code)]
    order_symbol: Symbol,

    #[allow(dead_code)]
    price_symbol: Symbol,

    bid_queue: OrderQueue<Symbol>,
    ask_queue: OrderQueue<Symbol>,
}

impl<Symbol> OrderBook<Symbol> {
    pub fn new(order_symbol: Symbol, price_symbol: Symbol) -> Self {
        OrderBook {
            order_symbol,
            price_symbol,
            bid_queue: OrderQueue::new(Side::Bid, 1000),
            ask_queue: OrderQueue::new(Side::Ask, 1000),
        }
    }

    pub fn handle_request(&mut self, order_request: OrderRequest<Symbol>) -> bool {
        let queue = match order_request.order.side {
            Side::Ask => &mut self.ask_queue,
            Side::Bid => &mut self.bid_queue,
        };

        queue.insert(order_request)
    }

    pub fn current_spread(&mut self) -> Option<(u64, u64)> {
        let bid = self.bid_queue.peek()?.price;
        let ask = self.ask_queue.peek()?.price;

        Some((bid, ask))
    }
}
