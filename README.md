# eyg
Language ideas

## Mission
- Change the concept of legacy apps to established apps
- Optimise longterm developer happiness
- Careful use of language. Facts and Actors make a better vocabulary that Classes and instances

> Personification is misleading and harms adoption of immutability
> Silly comment like destroy the person and create a new one with a different name.

> In a system a customer is a role. A role has expectations, expectations are replaced not updtated. therefore a role is immutable and has versions. The customer is not an immutable person but an immutable role.


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

### Facts and Actors (Episodic time model)

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

look at erl2
