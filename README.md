### nozomi

experimental order-matching engine in Rust, utilizing [Axiom](https://github.com/rsimmonsjr/axiom).

> What's this for?

To learn more about [order-matching systems](https://en.wikipedia.org/wiki/Order_matching_system) and their implementation details which can be used to facilitate exchanges for all kinds of assets.

This is also an example project to test the capabilities and further the development on [Axiom](https://github.com/rsimmonsjr/axiom), which is a scalable + ergonomic actor model for Rust.

---

Desires (to maybe implemented)

- network actors feed into OrderBookActors
- Each OrderBook is managed by their own special OrderBookActor.
- main actor responsible for proxying messages to the correct OrderBookActor (replaced with a registry?)
- highly-visible statistics around the system (GUI / API)

Order functionality:

- market / limit orders
- partial fill orders
- editing limit orders
- expiring limit orders
- cancelling limit orders

TODOs:

- Extract OrderBookActor from Engine
- Add testing of OrderBookActor
- Add support for partially filled orders
- Add support for limit orders
- Add support for grabbing OrderBook data
- Add support for OrderBookSupervisor (which spawns order book actors for all symbols traded in system)
- Add data retention if OrderBookActor crashes, or system is turned off. (rocksdb or?)

architecture:

- core::order_book
- core::orders
- core::tracker

- engine::actors::{OrderBookActor, OrderBookSupervisor}

upstream:

- Look into implementing monitors/links in Axiom (supervisor primitives)
- Build supervisor actors from monitor/link primitives