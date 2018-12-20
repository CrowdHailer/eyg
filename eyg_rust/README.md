# Eyg (Rust)

## Actor implementation for Rust.

Inspired by a desire for typed actors when working with Elixir.
Note, although inspired by erlang/OTP process model it is in no way an error to reproduce that.

All Actors/Workers/Processes in the system can be implemented as pure functions.
In this regard the project may be closer to an executable model checker than other Actor implementations.

#### Mailboxes

These are they key abstraction for this library.
All mailboxes are defined by the type of the messages they can receive,
and an identifier that separates mailboxes of the same kind of messages.

*The type of messages a mailbox can receive is constant over time.
In a distributed system messages can always arrive late so it is not possible to change what is acceptable.*

#### Workers

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

#### Runtime

Responsible for delivering dispatched messages to the appropriate worker.

- OrderedRuntime: Runs through each message in order.
- ExhaustiveRuntime: Check every message ordering.

<!-- Janken Exhausive option -->

## Examples

### Abacus - Counters

Main process and continuation

### Janken

Exhausive runtime

### Watercooler

side-effects/driver

## Notes

### Should any message types be built in.

e.g. Timeout.

Also Sys/GenCall/Monitor (I think all these can be built on top of the base level)

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
