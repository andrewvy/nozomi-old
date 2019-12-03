use crate::core::orders::{Order, OrderRequest, OrderType, Side};

use axiom::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Symbol {
    USD,
    ABC,
}

pub struct Data {
    orders: Vec<OrderRequest<Symbol>>,
}

impl Data {
    fn handle_new_order_request(mut self, request: OrderRequest<Symbol>) -> ActorResult<Self> {
        dbg!(request);

        self.orders.push(request);

        Ok(Status::done(self))
    }

    async fn handle(self, _context: Context, message: Message) -> ActorResult<Self> {
        if let Some(sys_msg) = message.content_as::<SystemMsg>() {
            match &*sys_msg {
                SystemMsg::Start => {}

                // This code runs each time a monitored `Game` actor stops. Once all the actors are
                // finished, the average final results of each game will be printed and then the
                // actor system will be shut down.
                SystemMsg::Stopped { .. } => {}
                _ => {}
            }
        }

        if let Some(msg) = message.content_as::<OrderRequest<Symbol>>() {
            self.handle_new_order_request(*msg)
        } else {
            Ok(Status::done(self))
        }
    }
}

pub fn start() {
    let system = ActorSystem::create(ActorSystemConfig::default().thread_pool_size(2));

    let data = Data {
        orders: Vec::with_capacity(50),
    };

    let aid = system
        .spawn()
        .name("USD/ABC")
        .with(data, Data::handle)
        .unwrap();

    let order_request = OrderRequest {
        order: Order {
            order_id: 1,
            order_symbol: Symbol::ABC,
            price_symbol: Symbol::USD,
            side: Side::Bid,
            price: 1_0000,
            quantity: 1,
        },
        order_type: OrderType::Market,
    };

    aid.send(Message::new(order_request)).unwrap();

    system.await_shutdown(None);
}
