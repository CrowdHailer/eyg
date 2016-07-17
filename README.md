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

- Single integer/fraction type
- types as sets of acceptable binary values
  - extend type system onto the wire, Joe Armstrong UBF
  - Interfaces and side effect referencing methods/objects are modled as a set of capabilities(interface/port) and providers(adapters)
  - simplified generation for property based testing
- abstract the computer but not the communication protocols
- separate ok/error for invalid values from ok/error for side effect failures, one is memoizable the other is not
