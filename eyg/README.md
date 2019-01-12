# Eyg (Rust)

## Actor implementation for Rust.

Inspired by a desire for typed actors when working with Elixir.
*Note, although inspired by erlang/OTP process model this project is not an attempt to reproduce that process model.*

All Actors/Workers/Processes in the system can be implemented as pure functions.
In this regard the project may be closer to an executable model checker than other Actor implementations.

## Example

In this example we implement a simple counter.

### Create a type for the state of a counter

A simple counter needs no more state than the running total.

```rust
pub struct CounterState(i32);
```

### Define messages that may be sent to a counter.

```rust
// This might be better called the Channel/Protocol,
// it is the description of how to send messages.
use eyg::worker::Worker;
use eyg::envelope::Mail;

pub enum CounterMessage {
    Increment,
    // may messages might be added in the future ...
}

impl<S> Worker<CounterMessage, S> for CounterState {
    fn new() -> Self {
        CounterState(0)
    }

    fn handle(self, _message: CounterMessage) -> (Mail<S>, Self) {
        (vec![], CounterState(self.0 + 1))
    }
}
```
*comment about moving new out or leaning on Default trait*

### Establish workers at a type of address

```rust
#[derive(Hash, Eq, PartialEq)]
pub struct CounterAddress(i32);

use std::collections::HashMap;

// This is how we specify how to look up workers in a general system implementation.
// NOTE possibility to handle other types of actor systems.
impl typemap::Key for CounterAddress {
    type Value = HashMap<CounterAddress, CounterState>;
}
```

### Apply a set of messages to a system of actors

```rust
fn main() {
    use eyg::envelope::{Envelope, Mail};
    use eyg::system::GeneralSystem;
    use eyg::runtime::OrderedRuntime;

    let mut system = GeneralSystem::new();

    let envelope = Envelope{
        address: CounterAddress(1),
        message: CounterMessage::Increment
    };

    let mail = vec![Box::new(envelope),Box::new(envelope)];

    let mut runtime = OrderedRuntime::new(system);
    runtime = runtime.dispatch(mail);
}
```

Need to add notes about error handling.
Need io integration
Need exhaustive testing

## Glossary

#### Worker

implementing the `Worker` trait indicates that a given state (CounterState) can respond to a certain type of messages.

A state may act as a worker for more than one kind of message,
although it is common to implement only one. *comment about the continuation style workers.*

*The type of messages a worker can process is constant over time.
In a distributed system messages can always arrive late so it is not possible to change what messages are acceptable.*

One worker will process sequentially all the messages received in a mailbox.
High concurrency is achieved by having many mailboxes.

State is persisted between handling messages, for each worker.
In response to a new message the worker returns

- An updated iteration of the state
- A collection of messages to be dispatched

*Note works are not spawned, sending a message to a new address starts workers on demand.
To control sending messages to a process the generation of addresses (mailbox id's) can be controlled.*

*Using the ownership system in Rust it is possible to create single use addresses,
for example guaranteeing that a ping is only replied to once.*

#### System

The collection of all workers.

A General purpose system is provided that is build on top of typemap, or your own system can be built.

##### Notes

The home trait specifies that a worker can be found for a certain message type

```rust
pub trait Home<M: Mailbox, S, W: Worker<M, S>> {
    fn get(&mut self, id: M::Id) -> W;
    fn put(&mut self, id: M::Id, worker: W);
}
```

#### Runtime

Responsible for delivering dispatched messages to the appropriate worker.

- OrderedRuntime: Runs through each message in order.
- ExhaustiveRuntime: Check every message ordering.

## Examples

### Abacus - Counters

Main process and continuation

### Janken

Exhausive runtime

### Embedded hello world

Smallest Runtime

### Watercooler

side-effects/driver

### EygIO

Create a separate crate for io implementations because the system will need to be extensible in this way. and this will prevent implementing parameterised traits.

## Notes

### Should any message types be built in.

e.g. Timeout.

Also Sys/GenCall/Monitor (I think all these can be built on top of the base level)

#### More effecient linear Runtime

When applying a message to a worker.

a) group at sender by address so all messages sent in one action to one actor, will be processed together. Nice for grouping log lines for examples.
(Though that grouping might happen anyway)

b) Group by Runtime
Take the leading message find all with the same address and deliver at once,
efficient if processes can be stashed or sleep. Might be a way to have essentally unlimited Process and not Crush memory, although still clearing up things like server processes might be nice

### Should Worker be able to handle more than one message type

### Have some property of Address that says if it can be recreated

Some actors should die when no other actor has a reference to them,
however some more persistent actors might be addressible over the network.

It's possible that modelling everything as the first type and that a worker can process things for an entity at a different abstraction level.
This Would probably be the simplest to implement

### Mailboxes cannot change message type

How to represent an actor that might change the messages it can accept over time,
for example certain messages are received only during setup?

One solution could be to have a set up process that starts a second process in the running state
and returns references to this new process in place of it's self.
This might work in a HTTP router where different routes need to work with different messages.

### System configuration

Because Systems are implemented separatly for each application configuration can be handled any way.
The main choice is between:

- passing configuration from the system state, as an argument of init.
- Sending required configuration in the first message received.
- Workers can also be manually inserted into the system before starting,
  in such cases the init can throw an error.

### Having to always implement Home/deliver

This might be a good thing.
Things that look like Actors but are out side the system can still be delivered too.

### Enumerate Runtime

If each address is hashable then we can put all the messages in a map.
the pop of the front of each list.
This will reduce the number of options that we need to test, don't need to try all the combos that deliver same message order per actor

### Workers don't need to be individual

As long as the order of messages in the mailbox can be agreed and processing messages is deterministic,
then then can be run on more than one machine. perhaps to reduce latency.

### Session types

Should be possible if messages are delivered in order, otherwise the session is very loosly specified as to not be valuable.
Could just throw runtime exceptions if violated

### Renaming

The core project is the specification of a parralel program,
it could be renamed `comms`, `eyg_core`, `eyg_model` etc.
The top level eyg project coul just pull in this dependency
