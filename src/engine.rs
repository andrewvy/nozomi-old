use std::time::SystemTime;

use crate::core::order_book::OrderBook;
use crate::core::orders::{Order, OrderRequest, OrderType, Side};

use axiom::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Symbol {
    USD,
    ABC,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum OrderBookCommands {
    NewRequest(OrderRequest<Symbol>),
    LogCurrentSpread,
}

pub struct OrderBookActor {
    order_book: OrderBook<Symbol>,
}

impl OrderBookActor {
    fn handle_new_order_request(mut self, request: OrderRequest<Symbol>) -> ActorResult<Self> {
        self.order_book.handle_request(request);

        Ok(Status::done(self))
    }

    fn log_current_spread(mut self) -> ActorResult<Self> {
        dbg!(self.order_book.current_spread());

        Ok(Status::done(self))
    }

    async fn handle(self, _context: Context, message: Message) -> ActorResult<Self> {
        if let Some(sys_msg) = message.content_as::<SystemMsg>() {
            match &*sys_msg {
                SystemMsg::Start => {}
                SystemMsg::Stopped { .. } => {}
                _ => {}
            }
        }

        if let Some(msg) = message.content_as::<OrderRequest<Symbol>>() {
            self.handle_new_order_request(*msg)
        } else if let Some(msg) = message.content_as::<OrderBookCommands>() {
            match *msg {
                OrderBookCommands::NewRequest(request) => self.handle_new_order_request(request),
                OrderBookCommands::LogCurrentSpread => self.log_current_spread(),
            }
        } else {
            Ok(Status::done(self))
        }
    }
}

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
        order_type: OrderType::Market,
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
        order_type: OrderType::Market,
        timestamp: SystemTime::now(),
    };

    aid.send(Message::new(OrderBookCommands::NewRequest(bid)))
        .unwrap();

    aid.send(Message::new(OrderBookCommands::NewRequest(ask)))
        .unwrap();

    aid.send(Message::new(OrderBookCommands::LogCurrentSpread))
        .unwrap();

    let new_ask = OrderRequest {
        order: Order {
            id: 3,
            order_symbol: Symbol::ABC,
            price_symbol: Symbol::USD,
            side: Side::Ask,
            price: 1_5000,
            quantity: 2,
        },
        order_type: OrderType::Market,
        timestamp: SystemTime::now(),
    };

    aid.send(Message::new(OrderBookCommands::NewRequest(new_ask)))
        .unwrap();

    aid.send(Message::new(OrderBookCommands::LogCurrentSpread))
        .unwrap();

    system.await_shutdown(None);
}
