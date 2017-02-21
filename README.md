# eyg
**Trying to realise an "Order of Magnitude" improvement in productivity.**
I think it is possible but that more than one issue needs to be solved to realise it.

framework -> modular first. from hanami

## Mission
- Change the concept of legacy apps to established apps
- Optimise longterm developer happiness
- Extend domain knowledge to include time.
- Careful use of language. Facts and Actors make a better vocabulary that Classes and instances

> Personification is misleading and harms adoption of immutability
> Silly comment like destroy the person and create a new one with a different name.

> In a system a customer is a role. A role has expectations, expectations are replaced not updtated. therefore a role is immutable and has versions. The customer is not an immutable person but an immutable role.

> The resolution of conflicts and requirement for coordination are both domain issues. Therefore a separate database is a bad abstraction and in fact we need tools to ensure data consistency over time.

> Cache invalidation is hard because it is the wrong abstraction. A cache should never need to be invalidated as facts never go off.


Function from the point of a great story
what - function body
who - interface signature
when - example
why - code comment


Actor is a bad name. I like employee or even worker or participant.
Actors can act concurrently often only one of each type.
single librarian looks after a catalogue of books.1

## Details

### Functional vs OOP

Functional starts from pure concepts and works down to providing implementations. OOP was the result of working up from instructions that manipulate registers. OOP is optimised for users who understand computers but the best programs should not require any knowledge of the implementation of the execution.

Logic programming looks like it starts with even higher abstractions an so might be an even better starting point.

There is a convergence in language design when hard problems are involved. make the most of it https://www.info.ucl.ac.be/~pvr/flopsPVRarticle.pdf

### Facts and Actors (Episodic time model)
https://youtu.be/fhOHn9TClXY?t=750 computer as an atom of meaning

An entity is a timeseries of facts with a common unique identifier.
The evolution of time should be modeled as a pure function on the history of all causal events on interacting timelines.

Rick Hickey talks about the episodic time model, but it needs extending to allow concurrency.

### All side effects as changes to a state

Model of mocking services
client <--> server <--> service.

New model
peer <--> db <--> DB <--> worker

regard random numbers as an array in the database.
Requests to send email should be added to the data store.

### Pure messaging semantics

```
Math({:div, 5, 3})
```

send module this message
```
fn ({:div, 5, 3}) ->
  $Creator.send("hi")
  $STDOUT.send("hi")
  $STDERR.send("ooh")
  $STDOUT.send("bill")
  newstate

{newstate, {$Creator: ["hi], $STDOUT: ["hi", "bill"]}
```

in tests you can pass the test pid instead of a module and see the messages that were sent.
Make *stubbing?* a generic part of the language.

This looks like it should be called the message monad. process references that you do not pass should be macros.

- Single integer/fraction type
- types as sets of acceptable binary values
  - extend type system onto the wire, Joe Armstrong UBF
  - Interfaces and side effect referencing methods/objects are modled as a set of capabilities(interface/port) and providers(adapters)
  - simplified generation for property based testing
- abstract the computer but not the communication protocols
- separate ok/error for invalid values from ok/error for side effect failures, one is memoizable the other is not
- pattern match on regex's and sets containing a value. Make pattern matching a macro that is extensible?!
- Make it possible to return a match object. some kind of curried function

```elixir
a = Task.do(stuff)
b = Task.do(stuff)

receive do
  a(foo) ->
    IO.puts "#{foo} wins"
  b(bar) ->
    IO.puts "#{bar} wind"
end
```
```elixir
m = %{name: $var, age: 30}
m(var: x) = %{name: "dave", age: 30}
# also
m(var: "brian") = %{name: "dave", age: 30}
# NO match
```
### Well describe protocols

ask(Question) -> reply(Answer)
subscribe(Channel) -> notify(Update)
stream(content) -> chunk(section)

google RPC vs request/response.


look at erl2

- Should make nondeterminism impossible, look at bloom and lasp
- tools to make causality tracking easy are needed to allow causality in the application layer http://blog.jessitron.com/2016/09/provenance-and-causality-in-distributed.html
- causality tracking and a large enough quorum allows persisten DB to be dropped. Have a client that replicates the live data to a db but is a minor player.

- no special primitives. set equally important as list. binary(string) is only primitive.

have a map syntax which returns option type but is set up with direct types

```
m = %{val: 5}
m.val = maybe[int]
```

try `.cz` domain for causality -> because -> cz

possibly switch entirly to green blue deploy in stateful ring remove relups.

OR

have type system check the possibility of relups.

have messages sent all through a loop using genserver reply only. or have all messages to be sent returned in a map. one list is the list of messages to add to the log.
having them returned at the end allows it to be considered as a transaction.

Should a process write to its own state or have the ability to write to the mailbox of another.

Need a Javascript example with email sending, random numbers and a global uniqueness request.

Need consistency between startup as a script, as a new node as a process.

# Patterns as Types.

This could be a thing

## Nirvana
1. Immutability, domain understanding of facts and actors.
2. EventStore/redux, there are no caches to invalidate in an event log.
3. N-temporal eventstore/causality tracking, true representation of the clients actions distribution was always there.
4. Eventlog filterering/projections, provide actors with usable subsets of all history for decision making(both in time and space)
5. Intention recording and corroboration. Instead of calls to remote services worker pattern from pouchdb.
6. Enumerated program outcomes, logic programing with a time model allows effects of raceconditions to be ascertained.

CRDT's should be a stable point solution to enumerating program outcomes.
Most useful resources on this are disorderly programming.

https://disorderlylabs.github.io/

CALM add only facts

program state saveable for debugging. see eve example. state should enclude sha of source file.

checkout optional dependencies ie Rust ++ Cargo
