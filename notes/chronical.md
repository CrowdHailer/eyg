## 2018-12-27

https://github.com/CrowdHailer/eyg/commit/32c01b1dc140709bf2ddf646ac0fe5ca5abc7970#diff-10fdf5474fc54df33c10b0fb3b2e87a5L8

Define a `Mailbox` as a trait with two associated types `Id` and `Message`.
Define a `Worker<M: Mailbox>`

*A better name for mailbox might be channel or protocol.
A better name for id might be address.*

This is not an egonmic model of actors.
- A single worker can only accept a single type of message.
- Any sender must be able to create the message type set by the worker.
  This limits extension where a singe worker receives messages from two separate sources.

This was fixed by replacing the `Mailbox` trait with an `Envelope` struct.
An `Envelope` is parameterised by an address type and a message type.

Envelopes can be created with the same address type but different message types,
allowing a single worker to process more than one type of message.
