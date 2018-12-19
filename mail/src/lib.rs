// Everything is based around handling a message type,
// it is important that more than one type of message can exist in the system.
// For example a Counter might have a message type that is an enum of Increment and Reset

// This might be better called the Channel/Protocol, it is the description of how to send messages.
// Instead of mailbox.id consider channel.partition.
pub trait Mailbox {
    type Id: Copy;
    type Message: Copy;
}
// An envelope is used to deliver messages to a specific mailbox.
// NOTE two envelopes with the same id but different type are for different actors.
#[derive(Copy, Clone)]
pub struct Envelope<M: Mailbox> {
    // Changing the id to a parameterised type means that Home needs to take yet another parameter, ideally it would just take a mailbox as parameter
    pub id: <M as Mailbox>::Id,
    pub message: <M as Mailbox>::Message
}

// Every envelope in the system has to get marked as deliverable within a given system.
// NOTE It might be possible to have a default implementation that creates a deliver<M> method that calls system.get
// So that each envelope can trivially implement the delivery protocol
// NOTE It's probably more idiomatic to mutate the system and return mail/todos but sort after first version
// NOTE rename system -> cohort???
pub trait Deliverable<S> {
    fn deliver(&self, system: S) -> (Todos<S>, S);
}

// A list of any of the messages that can be delivered in the system
// This call is dynamic so that new message types can be added without having to update existing code
pub type Todos<S> = Vec<Box<dyn Deliverable<S>>>;

// A worker that processes all the message of a given type with the same actor id.
trait Worker<M: Mailbox, S>: Sized {
    fn handle(self, message: &M::Message) -> (Todos<S>, Self);
}
// The home trait specifies that a worker can be found for a certain message type
// NOTE I think by using something like typemap it should not be required to have a user defined MySystem to store actors
// It would require typemap to only hold things that implement actor,
trait Home<M: Mailbox, S, W: Worker<M, S>> {
    fn get(&mut self, id: M::Id) -> W;
    fn put(&mut self, id: M::Id, worker: W);
}

pub struct Runtime<S>(S);
impl<S> Runtime<S> {
    pub fn dispatch(mut self, mut envelopes: Todos<S>) -> Self {
        // TODO this works in a funny order, pops the last message and then sticks new messages on the end.
        // it should pop from one end and append messages on the other.
        // Technically no order guarantees are given so this doesn't matter but it's weird.
        while let Some(e) = envelopes.pop() {
            let mut x = e.deliver(self.0);
            envelopes.append(&mut x.0);
            self.0 = x.1;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Envelope;
    use crate::Mailbox;
    use crate::Worker;
    use crate::Home;
    use crate::Deliverable;
    use crate::Todos;
    use crate::Runtime;

    #[derive(Debug, Copy, Clone)]
    struct Foo();
    impl Mailbox for Foo {
        type Id = i32;
        type Message = Self;
    }
    #[derive(Debug, Copy, Clone)]
    struct Bar();
    // Use unit as the Id because there should only be one Bar process at a time
    impl Mailbox for Bar {
        type Id = ();
        type Message = Self;
    }
    #[derive(Debug)]
    struct FooWorker(i32);
    #[derive(Debug, Copy, Clone)]
    struct BarWorker();
    impl Worker<Foo, MySystem> for FooWorker {
        fn handle(self, message: &Foo) -> (Todos<MySystem>, Self) {
            println!("Foo processed {:?}", message);
            (
                vec![Box::new(Envelope::<Bar>{id: (), message: Bar()})],
                FooWorker(self.0 + 1)
            )
        }
    }
    impl Worker<Bar, MySystem> for BarWorker {
        fn handle(self, message: &Bar) -> (Todos<MySystem>, Self) {
            println!("Bar processed {:?}", message);
            (vec![], self)
        }

    }
    use std::collections::HashMap;
    #[derive(Debug)]
    pub struct MySystem {
        foos: HashMap<i32, FooWorker>,
        bar: BarWorker
    }
    impl MySystem {
        pub fn new() -> MySystem {
            MySystem{foos: HashMap::new(), bar: BarWorker()}
        }
    }
    impl Home<Foo, MySystem, FooWorker> for MySystem {
        fn get(&mut self, id: <Foo as Mailbox>::Id) -> FooWorker {
            self.foos.remove(&id).unwrap_or(FooWorker(0))
        }
        fn put(&mut self, id: <Foo as Mailbox>::Id, worker: FooWorker) {
            self.foos.insert(id, worker);
        }
    }
    impl Home<Bar, MySystem, BarWorker> for MySystem {
        // Perhaps parameterising the get fn (rather than the impl will make it easier for a general deliverable)
        fn get(&mut self, (): <Bar as Mailbox>::Id) -> BarWorker {
            self.bar
        }
        fn put(&mut self, (): <Bar as Mailbox>::Id, worker: BarWorker) {
            self.bar = worker;
        }
    }

    // <MySystem as Home<Foo>>
    // Write this as system.process::<Foo>(self)
    impl Deliverable<MySystem> for Envelope<Foo> {
        fn deliver(&self, mut my_system: MySystem) -> (Todos<MySystem>, MySystem) {
            let id: <Foo as Mailbox>::Id = self.id;
            let worker: FooWorker = my_system.get(id);
            let (outbound, new_worker) = worker.handle(&self.message);
            my_system.put(id, new_worker);
            (outbound, my_system)
        }
    }
    impl Deliverable<MySystem> for Envelope<Bar> {
        fn deliver(&self, mut my_system: MySystem) -> (Todos<MySystem>, MySystem) {
            let id: <Bar as Mailbox>::Id = self.id;
            let worker: BarWorker = my_system.get(id);
            let (outbound, new_worker) = worker.handle(&self.message);
            my_system.put(id, new_worker);
            (outbound, my_system)
        }
    }


    // Implement a TCP ech server
    // Implement a Fanout and Counter and Main
    // Implement a GenCall with one use reference
    // Rock Paper Scissors
    // Show state changing by delegation
    // Write down thoughts about system level messages
    // Timeouts by sending two messages, note that you don't need to wait because switching the ordering does that
    // Good example of one of the reasons to use it.
    // Should be able to implement duplication/loss/reordering as a wrapper for handler.
    // return message in list and pass to wrapped handler for duplication, do nothing for loss.

    // If the Actor has the state of a closure you should be able to write all sorts of possible unnecessaty helpers
    // Monad.flat_map(Logger.debug("s"), {|_ok| -> do the rest})
    // Gen::{Call, Sys, Cast}
    #[test]
    fn it_works() {
        let runtime = Runtime(MySystem::new());
        let e1 = Envelope::<Foo>{id: 1, message: Foo()};
        let e2 = Envelope::<Bar>{id: (), message: Bar()};
        let envelopes: Todos<MySystem> = vec![Box::new(e1), Box::new(e2), Box::new(e1)];
        let runtime = runtime.dispatch(envelopes);
        let runtime = runtime.dispatch(vec![]);
        println!("{:?}", runtime.0);
        assert_eq!(2 + 2, 3);
    }
}
