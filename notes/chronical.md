## 2019-01-19

When a trait might be implemented multiple times on a type it is best not to specify how to create that type.

For example

```rs
pub trait Worker<M, S> {
    fn new() -> Self;
    fn handle(self, message: M) -> (crate::envelope::Mail<S>, Self);
}
```

Becomes

```rs
pub trait Handler<M, S> {
    fn handle(self, message: M) -> (Mail<S>, Self);
}
```

Making this change also allows considerations around configuration to be postponed.

Should configuration be a system global value that is passed to each worker at startup, or should the required information be part of the first messages.

## 2019-01-13

```rust
pub trait GenServer<S> {
    fn handle_call<A, R>(self, caller: Caller<A>, request: R) -> (Mail<S>, Self);
}
```
- S: System
- A: Address of Caller
- R: Request message

This trait does NOT work as a specification of a GenServer.
Implementing this trait for a given system implies that the server can handle a call with ANY address and request message.

Instead use a `GenCall<Req, Resp>` trait.
*NOTE this is not generalised by system.*
Implementing this trait for specific request and response defines a protocol.

```rust
use std::collections::HashMap;
pub trait GenCall<Req, Resp> {
    fn handle_call<A, W>(self, caller: Caller<A, Resp>, request: Req) -> (Mail<GeneralSystem>, Self)
    where
        W: Worker<Reply<Resp>, GeneralSystem> + 'static,
        A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash;
}
```

This implementation solves the problems above,
however it requires a lot of specification to match addresses and workers.

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
