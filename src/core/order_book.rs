use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::time::SystemTime;

use crate::core::orders::{Order, OrderRequest, OrderType, Side};

#[derive(Clone)]
struct OrderIndex {
    id: u64,
    price: u64,
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

/// Encapsulates a priority queue of Orders, ordered by OrderIndex.
pub struct OrderQueue<Symbol> {
    index_queue: BinaryHeap<OrderIndex>,
    orders: HashMap<u64, Order<Symbol>>,

    #[allow(dead_code)]
    order_side: Side,
}

impl<Symbol> OrderQueue<Symbol> {
    /// Creates a new OrderQueue.
    pub fn new(side: Side, capacity: usize) -> Self {
        OrderQueue {
            index_queue: BinaryHeap::with_capacity(capacity),
            orders: HashMap::with_capacity(capacity),
            order_side: side,
        }
    }

    /// Returns the highest priority Order, `None` if the queue is empty.
    pub fn peek(&mut self) -> Option<&Order<Symbol>> {
        if let Some(order_id) = self.get_best_order_id() {
            if self.orders.contains_key(&order_id) {
                self.orders.get(&order_id)
            } else {
                self.index_queue.pop()?;
                self.peek()
            }
        } else {
            None
        }
    }

    /// Inserts in a new OrderRequest into the queue.
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

    /// Removes the highest priority Order from the queue, `None` if the queue is empty.
    pub fn pop(&mut self) -> Option<Order<Symbol>> {
        if let Some(order_id) = self.index_queue.pop() {
            if self.orders.contains_key(&order_id.id) {
                self.orders.remove(&order_id.id)
            } else {
                self.pop()
            }
        } else {
            None
        }
    }

    /// Amends a given order id, price, and updates the stored Order.
    pub fn amend(&mut self, id: u64, price: u64, order: Order<Symbol>) -> bool {
        if self.orders.contains_key(&order.id) {
            self.orders.insert(order.id, order);
            self.rebuild_index(id, price, SystemTime::now());
            true
        } else {
            false
        }
    }

    pub fn alter_head(&mut self, new_order: Order<Symbol>) -> bool {
        if let Some(order_id) = self.get_best_order_id() {
            if self.orders.contains_key(&order_id) {
                self.orders.insert(order_id, new_order);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Gets the highest priority order id from the queue, `None` if the queue is empty.
    fn get_best_order_id(&self) -> Option<u64> {
        match self.index_queue.peek() {
            Some(order_key) => Some(order_key.id),
            None => None,
        }
    }

    fn rebuild_index(&mut self, id: u64, price: u64, timestamp: SystemTime) {
        let mut new_orders = self.index_queue.clone().into_vec();

        new_orders.retain(|order_index| order_index.id != id);

        new_orders.push(OrderIndex {
            id,
            price,
            timestamp,
            order_side: self.order_side,
        });

        let new_queue = BinaryHeap::from(new_orders);

        self.index_queue = new_queue;
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

#[derive(Debug)]
pub enum OrderBookResponse {
    Filled,
    PartiallyFilled,
    Unfulfilled,
    Rejected,
}

impl<Symbol> OrderBook<Symbol>
where
    Symbol: Copy + std::fmt::Debug,
{
    pub fn new(order_symbol: Symbol, price_symbol: Symbol) -> Self {
        OrderBook {
            order_symbol,
            price_symbol,
            bid_queue: OrderQueue::new(Side::Bid, 1000),
            ask_queue: OrderQueue::new(Side::Ask, 1000),
        }
    }

    // @TODO(vy): return filled prices/quantities in response
    pub fn handle_request(&mut self, order_request: OrderRequest<Symbol>) -> OrderBookResponse {
        let (request_queue, order_queue) = match order_request.order.side {
            Side::Ask => (&mut self.ask_queue, &mut self.bid_queue),
            Side::Bid => (&mut self.bid_queue, &mut self.ask_queue),
        };

        match order_request.order_type {
            OrderType::Market => {
                if let Some(order) = order_queue.peek().cloned() {
                    if order_request.order.quantity < order.quantity {
                        // If this request can be immediately fulfilled by the given Order.
                        order_queue.alter_head(Order {
                            id: order.id,
                            order_symbol: order.order_symbol,
                            price_symbol: order.price_symbol,
                            price: order.price,
                            quantity: order.quantity - order_request.order.quantity,
                            side: order.side,
                        });

                        OrderBookResponse::Filled
                    } else if order_request.order.quantity > order.quantity {
                        // Else, this request can only be partially fulfilled.
                        // @TODO(vy: add order request to orderbook)
                        OrderBookResponse::PartiallyFilled
                    } else {
                        // Or, this request can be perfectly matched to an order.
                        order_queue.pop();
                        OrderBookResponse::Filled
                    }
                } else {
                    request_queue.insert(order_request);
                    OrderBookResponse::Unfulfilled
                }
            }
            OrderType::Limit => {
                request_queue.insert(order_request);
                OrderBookResponse::Unfulfilled
            }
        }
    }

    pub fn current_spread(&mut self) -> Option<(u64, u64)> {
        let bid = self.bid_queue.peek()?.price;
        let ask = self.ask_queue.peek()?.price;

        Some((bid, ask))
    }
}
