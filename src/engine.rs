use std::time::SystemTime;

use crate::core::order_book::OrderBook;
use crate::core::orders::{Order, OrderRequest, OrderType, Side};

use crate::engine::actors::{OrderBookActor, OrderBookCommands, Symbol};

use axiom::prelude::*;

pub mod actors;

pub fn start() {
    let system = ActorSystem::create(ActorSystemConfig::default().thread_pool_size(2));

    let order_book_actor = OrderBookActor {
        order_book: OrderBook::new(Symbol::ABC, Symbol::USD),
    };

    let aid = system
        .spawn()
        .name("USD/ABC")
        .with(order_book_actor, OrderBookActor::handle)
        .unwrap();

    let bid = OrderRequest {
        order: Order {
            id: 1,
            order_symbol: Symbol::ABC,
            price_symbol: Symbol::USD,
            side: Side::Bid,
            price: 1_0000,
            quantity: 1,
        },
        order_type: OrderType::Limit,
        timestamp: SystemTime::now(),
    };

    let ask = OrderRequest {
        order: Order {
            id: 2,
            order_symbol: Symbol::ABC,
            price_symbol: Symbol::USD,
            side: Side::Ask,
            price: 2_0000,
            quantity: 2,
        },
        order_type: OrderType::Limit,
        timestamp: SystemTime::now(),
    };

    aid.send_new(OrderBookCommands::NewRequest(bid)).unwrap();

    aid.send_new(OrderBookCommands::NewRequest(ask)).unwrap();

    // Should be (20000, 10000).
    aid.send_new(OrderBookCommands::LogCurrentSpread).unwrap();

    let new_ask = OrderRequest {
        order: Order {
            id: 3,
            order_symbol: Symbol::ABC,
            price_symbol: Symbol::USD,
            side: Side::Ask,
            price: 1_5000,
            quantity: 2,
        },
        order_type: OrderType::Limit,
        timestamp: SystemTime::now(),
    };

    aid.send_new(OrderBookCommands::NewRequest(new_ask))
        .unwrap();

    // Should now be (15000, 10000).
    aid.send_new(OrderBookCommands::LogCurrentSpread).unwrap();

    let market_order = OrderRequest {
        order: Order {
            id: 3,
            order_symbol: Symbol::ABC,
            price_symbol: Symbol::USD,
            side: Side::Bid,
            price: 0,
            quantity: 2,
        },
        order_type: OrderType::Market,
        timestamp: SystemTime::now(),
    };

    aid.send_new(OrderBookCommands::NewRequest(market_order))
        .unwrap();

    // Should go back to (20000, 10000), since the 15000 ask order was filled by our market order request.
    aid.send_new(OrderBookCommands::LogCurrentSpread).unwrap();

    system.await_shutdown(None);
}
