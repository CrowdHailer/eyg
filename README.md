# EYG

### Explicit concurrency for intelligible parallel programing.

See the [eyg](eyg) directory for the core project and introduction.

The rest of this repository consists of examples and experiments


- [Proposals](#Proposals)
  - Receive return Result(m, Nil) instead of Option(m)
  - Have a separate receive and receive_max function with different return types.
  - supervisor has a type of the child pid, the start function can take any run function that has the correct message type. try on task supervisor
  - Remove `BarePid` in favour of `Pid(Never)` (process.NoReturn, task.NoMessage)
  - Type safe link + monitor
  - Build `process.call` on top of channels.
  - Traits for sendable receivable
  - Introduce a Gen Actor concept
  - Rename Pid
  - Introduce tasks as a base concept
  - Proc naming for disterl, avoid as long as possible
  - Process Dictionary,  I think just a way to generate functions


## Notes

### Typed send and receive

Expanded here https://elixirforum.com/t/a-staticly-typed-api-for-sending-receiving-messages-in-gleam/31449

### self derived from receive

A receive function that took a pid (which should only be self) was too easy to pass other processes.
having a function named receive was more intention revealing, there might be a better name than self.

### Processes don't error at startup
- Naming - dealt with below
- Config validation, handled by the type system
- Checking external service, not best handled by startup because slow

Advantage is that the is no state transition from initializing to running.
Messages can be sent immediatly to a pid they will queue in mailbox which is normal.

One thing that can cause failures is naming processes.
This is a big global state problem and probably could always happen,
but at the registered process level a naming failure should be a hard crash.

Potential an Ignore response is ok, for example not starting a server if the PORT is not set.
There is no reason the supervisor can't handle this but it might be a nicer grouping of logic to put this in the particular process.
This is similar to the logic that lead to child_specs

Remove init_ack

Having the init and init_ack functionality causes alot of delay and the supervisor has to watch for more race conditions

### Naming + Registries

Aim to have Gleam without names, You might need root naming for connections between nodes using disterl. However that would not be the case for incoming network connections because the server can be started with references to all the required pids.

Registry return should be `Result(Option(Pid))` error for failure to communicate, None for no pid.

could there be a `new_or_create` / `update` function that takes the existing value and returns a new one, could be smart for already started behaviour
`pub fn register(registry: Registry(a, b), key: a, create: fn(Option(b)) -> Option(b)) -> Result(b, Nil)`

start can return Result(Pid, Pid) Ok is new Pid error is existing pid.

### Supervision

process always has to handle case it might be too slow and be killed, or node dies. let's go strait to kill!

Question does a supervisor need to return more than a pid,
might be helpful to return refernce to ets table etc.
If ets is started reference can be returned by a call to the pid in an init step


Complicates supervisors, they need to know how to get to pid, probably all children will end up with mapping to pid, as an identity function.

Might be a way to do Child {
Processs(fn -> pid)
More(fn -> term, fn(term) -> pid)
}

where term is returned when  starting child, but pid is monitored by supervisor.

5. return more than pid from supervisor, might want reference to ets table or some such. supervisors can make a single call to look up ets table and pass reference to all childrean.

Defining child spec on the child module doesn't work if parameterised at the supervisor level,
unless PermanentSupervisorChild(Permanent(run, blah))

#### Supervise process from Run only.
Note if spawning is extended to take a list of options this will need to be specified in the run spec

  ```rust
  pub fn spec(){
    Permanent(run, [TrapExit(Exit)])
  }
  ```

### Notes from OTP review

start_link functions must start the link, supervisors do not handle this
https://erlang.org/doc/man/supervisor.html

When terminating children supervisor unlinks and monitors, to make sure a child that has unlinked cannot cause the supervisor to hang
https://github.com/erlang/otp/blob/master/lib/stdlib/src/supervisor.erl#L896

The above leads to a race condition when killing supervisors, they must be given infinity time to kill their own children
After a timeout will always kill,
THis supervisor works on assumption that kill is ok

Using a receive it is possible to not wrap all custom messages in a Message block,
Doing so does allow one to forget to implement Down and Exit messages
It also allows owners of the address to send in the Down and exit messages

Proc lib defines init_ack, and funtions to wait for that, init_ack returns the pid in most cases
spawn_link does nothing but get parents
init_p simply sets parents in new process
start_link uses spawn link, awaits a reply from init_ack and then demonitors if appropriate

Gen module defines call, but also naming that I don't yet want to handle

## Proposals

### Remove BarePid

Makes it easier to write a process.equal(pid, pid) otherwise doesn't do much

Introduces the `Never` type.
The name `Never` is a convention, not creating it is not enforced.
It is possible that `Never` would also be used by processes that do not terminate.
At which point this type has multiple meanings.

This example would compile, technically no problem because the task never ends so message never sent
```rust
process.send(never_pid, neverending_task())
```
Other possibilities
```rust
let x: Never = dynamic.unsafe_cast
```

### Type safe links and monitors

Enforcing every worker process to handle DOWN messages, that might never arrive is verbose.
Also before completeness checking its just as easy to forget a case clause as it is to forget to add a down/exit message to a processes protocol.
Because this is also low level it is up to the implementer to handle down exit where necessary, all other types in a the protocol are checked.

There are arguments for not doing this,
one is simplicity
- For this version if a process might accept DOWN or EXIT messages the user needs to remember to add them to the protocol.
  This is to give maximum control to implementers of supervisors etc.
  Also this level of abstraction is infrequently changed so well tested libraries can be used.

https://github.com/midas-framework/midas/issues/20

If passing trap exit at spawn then lot's of empty lists passed to spawn functions,
not much of a problem if layers added above, also need child specs to include these args, if taking run function for supervisor.

receive can have a type of Nil, or Monitorable so that process.monitor(receive, pid) works only if it is set.
Actually not sure it can because the spawn function is generic and this Monitor option will just be one in a list.

```rust
process.handle_down(receive, )
process.trap_exit(receive, )
```
I think we can set it once and write a wrapper
```
pub fn trap_exit(receive, func){
  return fn(x) {
    case receive(x) {
      Exit(pid, b) -> func(pid, b)
      other -> other
    }
  }
}
```

use type to set it only once.

Still questions here
1. single monitor callback, or set one with every monitor call
2. trap_exit in spawn args or side effect in function, see other options that might be added


### Call on top of channels

This would enshrine the concept of channels, which might not be needed

It could be powerful if possible to listen to multiple channels.
I don't know if this is the case, I think it would require a dynamic list of values to match against in a receive block.
https://stackoverflow.com/questions/62319712/how-to-receive-on-a-list-of-possible-values-in-erlang-or-elixir

 Perhaps a monitor can be a channel with message type never, if a channel has build in monitor, then it can't be multi sender, but if it's multi sender it can never be DOWN. so the down message can be used as a close message,

##### Call on top of reply

It might be simpler to just have the concept of a reply, i.e. it's used once.
Would work for tasks and calls.

How do you know to handle the down message in a general purpose receive call.
This would make yielding on the task hang forever.
You could always remonitor, would need more than one ref handled in call, but probably ok

Would also work for init ack

```rust
let tuple(receive, resolve) = promise.new()

process.spawn(fn(){
  resolve(5)
})

let Ok(value) = receive(Infinity)
```


http://joeduffyblog.com/2015/11/19/asynchronous-everything/

Gen can be effecient use monitor to make ref as long as it demonitors in the case of a timeout.

### Gen Actors

implement an init and handle function.
You can have the handle always return a new handle, continuation style

```
fn handle(message) {
  tuple(handle, sendable)
}
```

List of sendable messages with dynamic envelopes

```rust
type Envelope {
  Envelope(Pid(Dynamic), Dynamic)
}

process.envelope(pid: Pid(m), message: m) -> Envelope

process.reply_envelope(from: From(m), message: m) -> Envelope {
  let From(ref, pid) = from
  let message = tuple(ref, message)
  envelope(pid, message)
}
```

```rust
handle(m, s) -> tuple(s, List(Envelope))
```

```rust
type Message(a) {
    Push(a)
    Pop(From(a))
}

pub fn handle(message, state) {
    case message {
        Push(new) -> {
          tuple([new, ..state], [])
        }
        Pop(from) -> {
            let tuple(top, state) = list.pop(state)
            tuple(state, [process.reply_envelope(from, top)])
        }
    }
}

pub fn init() {
    []
}

gen_server.spawn_link(init, handle)
```

### Rename Pid

I would like to rename the type `Pid(m)` to `Address(m)` and `BarePid` to Pid.
it's possible for `Pid(Foo)` and `Pid(Bar)` to point to the same process, one might be a reply

However I think from an understanding point of view for new people explaining that there are now typed pids is easier than explaining what an address is.

### Tasks as foundation

An actor can be considered a Task + Channels.
I experimented with having tasks return there completion value in the contents of a monitor message.
This was done by exiting with reason `{done, value}`.

- The beam considers this a non-normal exit value and so linked processes terminate.
  It would require an implicit supervision tree, or to start them unlinked
- Channels would need better support, and that might not be possible do to limits in guard clauses

There is prior art for tasks as the base unit

##### Tasks built on processes

These are possible, and probably useful.
They can send return value in a normal message,
with a reference which makes them usable as a channel that can be awaited on.

Tasks, don't listen for any down message.
Can start a worker who watched caller and kills task if the caller dies.

```rust
task.start_link(fn ... )
tast.start_supervised
```

Can't monitor process thats already dead, getting an idempotent yield is tricky

Run experiments here https://github.com/midas-framework/midas/commit/e45829314df133f82f74057e4e916b42eaf89f29

Start supervised can return a reference that is awaitable like a call response

### Processes without receive

Http body could be a channel, monitors are a channel.
http handler could have a receive function that takes HTTP body parts, need a mapper and potentially the ability to map other messages

### Pure message sending

Can this be ergonmic enough to be useful.


### Proc dictionary

```
let tuple(fetch, set) = process.storage(name: "foo")
```
