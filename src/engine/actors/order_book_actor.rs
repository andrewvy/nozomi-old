use crate::core::order_book::OrderBook;
use crate::core::orders::OrderRequest;

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
    pub order_book: OrderBook<Symbol>,
}

impl OrderBookActor {
    fn handle_new_order_request(mut self, request: OrderRequest<Symbol>) -> ActorResult<Self> {
        dbg!(self.order_book.handle_request(request));

        Ok(Status::done(self))
    }

    fn log_current_spread(mut self) -> ActorResult<Self> {
        dbg!(self.order_book.current_spread());
        dbg!(self.order_book.dump());

        Ok(Status::done(self))
    }

    pub async fn handle(self, _context: Context, message: Message) -> ActorResult<Self> {
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
