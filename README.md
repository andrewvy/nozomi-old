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