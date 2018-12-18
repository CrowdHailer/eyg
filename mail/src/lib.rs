// Everything is based around handling a message type,
// it is important that more than one type of message can exist in the system.
// For example a Counter might have a message type that is an enum of Increment and Reset

// An envelope wraps a message with an actor id that refers to which actor the message should be delivered.
// NOTE two envelopes with the same id but different type are for different actors.
// TODO parameterise the id type so it doesn't have to me an i32
// This more represents mailboxes, message spec + id, can traits have default structs.
// If so impl Mailbox<id, Message> {} could define an envelope
#[derive(Copy, Clone)]
pub struct Envelope<M> {
    // Changing the id to a parameterised type means that Home needs to take yet another parameter, ideally it would just take a mailbox as parameter
    pub id: i32,
    pub message: M
}

// Every envelope in the system has to get marked as deliverable within a given system.
// NOTE It might be possible to have a default implementation that creates a deliver<M> method that calls system.get
// So that each envelope can trivially implement the delivery protocol
// NOTE It's probably more idiomatic to mutate the system and return mail/todos but sort after first version
trait Deliverable<System> {
    fn deliver(&self, s: System) -> (Todos<System>, System);
}

// A list of any of the messages that can be delivered in the system
// This call is dynamic so that new message types can be added without having to update existing code
type Todos<S> = Vec<Box<dyn Deliverable<S>>>;

// A worker that processes all the message of a given type with the same actor id.
trait Worker<M, S>: Sized {
    fn handle(self, message: &M) -> (Todos<S>, Self);
}
// The home trait specifies that a worker can be found for a certain message type
// NOTE I think by using something like typemap it should not be required to have a user defined MySystem to store actors
// It would require typemap to only hold things that implement actor,
trait Home<M, S, W: Worker<M, S>> {
    fn get(&mut self, id: i32) -> W;
}

#[cfg(test)]
mod tests {
    use crate::Envelope;
    use crate::Worker;
    use crate::Home;
    use crate::Deliverable;
    use crate::Todos;

    #[derive(Debug)]
    struct Foo();
    #[derive(Debug)]
    struct Bar();
    struct FooWorker();
    struct BarWorker();
    impl Worker<Foo, MySystem> for FooWorker {
        fn handle(self, message: &Foo) -> (Todos<MySystem>, Self) {
            println!("Foo processed {:?}", message);
            (vec![Box::new(Envelope{id: 1, message: Bar()})], self)
        }
    }
    impl Worker<Bar, MySystem> for BarWorker {
        fn handle(self, message: &Bar) -> (Todos<MySystem>, Self) {
            println!("Bar processed {:?}", message);
            (vec![], self)
        }

    }
    use std::collections::HashMap;
    pub struct MySystem {
        foos: HashMap<i32, FooWorker>,
        bars: HashMap<i32, BarWorker>
    }
    impl MySystem {
        pub fn new() -> MySystem {
            MySystem{foos: HashMap::new(), bars: HashMap::new()}
        }
    }
    impl Home<Foo, MySystem, FooWorker> for MySystem {
        fn get(&mut self, id: i32) -> FooWorker {
            self.foos.remove(&id).unwrap_or(FooWorker())
        }
    }
    impl Home<Bar, MySystem, BarWorker> for MySystem {
        fn get(&mut self, id: i32) -> BarWorker {
            self.bars.remove(&id).unwrap_or(BarWorker())
        }
    }

    impl Deliverable<MySystem> for Envelope<Foo> {
        fn deliver(&self, mut my_system: MySystem) -> (Todos<MySystem>, MySystem) {
            let worker: FooWorker = my_system.get(self.id);
            // TODO put the updated worker back in the system
            let (outbound, _new_worker) = worker.handle(&self.message);
            (outbound, my_system)
        }
    }
    impl Deliverable<MySystem> for Envelope<Bar> {
        fn deliver(&self, mut my_system: MySystem) -> (Todos<MySystem>, MySystem) {
            let worker: BarWorker = my_system.get(self.id);
            worker.handle(&self.message);
            (vec![], my_system)
        }
    }


    #[test]
    fn it_works() {
        let e1 = Envelope{id: 1, message: Foo()};
        let e2 = Envelope{id: 1, message: Bar()};
        let mut envelopes: Todos<MySystem> = vec![Box::new(e1), Box::new(e2)];
        let mut my_system = MySystem::new();
        // for e in &envelopes {
        //     let (_, new_system) = e.deliver(my_system);
        //     my_system = new_system;
        // }
        // TODO this works in a funny order, pops the last message and then sticks new messages on the end.
        // it should pop from one end and append messages on the other.
        // Technically no order guarantees are given so this doesn't matter but it's weird.
        while let Some(e) = envelopes.pop() {
            let mut x = e.deliver(my_system);
            envelopes.append(&mut x.0);
            my_system = x.1;
        }

        let mut things = vec![1, 2, 3, 4];
        while let Some(n) = things.pop() {
            println!("{:?}", n);
            if n == 3 {
                let mut tmp = vec![5, 8];
                tmp.append(&mut things);
                things = tmp;
            }
        }
        assert_eq!(2 + 2, 3);
    }
}
