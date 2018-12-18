// Everything is based around handling a message type,
// it is important that more than one type of message can exist in the system.
// For example a Counter might have a message type that is an enum of Increment and Reset

// An envelope wraps a message with an actor id that refers to which actor the message should be delivered.
// NOTE two envelopes with the same id but different type are for different actors.
// TODO parameterise the id type so it doesn't have to me an i32
#[derive(Copy, Clone)]
pub struct Envelope<M> {
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
// TODO this needs to return a vector of deliverables not i32.
trait Worker<M>: Sized {
    fn handle(self, message: &M) -> (Vec<i32>, Self);
}
// The home trait specifies that a worker can be found for a certain message type
// NOTE I think by using something like typemap it should not be required to have a user defined MySystem to store actors
// It would require typemap to only hold things that implement actor, 
trait Home<M, W: Worker<M>> {
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
    impl Worker<Foo> for FooWorker {
        fn handle(self, message: &Foo) -> (Vec<i32>, Self) {
            println!("Foo processed {:?}", message);
            (vec![], self)
        }
    }
    impl Worker<Bar> for BarWorker {
        fn handle(self, message: &Bar) -> (Vec<i32>, Self) {
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
    impl Home<Foo, FooWorker> for MySystem {
        fn get(&mut self, id: i32) -> FooWorker {
            self.foos.remove(&id).unwrap_or(FooWorker())
        }
    }
    impl Home<Bar, BarWorker> for MySystem {
        fn get(&mut self, id: i32) -> BarWorker {
            self.bars.remove(&id).unwrap_or(BarWorker())
        }
    }

    impl Deliverable<MySystem> for Envelope<Foo> {
        fn deliver(&self, mut my_system: MySystem) -> (Todos<MySystem>, MySystem) {
            let worker: FooWorker = my_system.get(self.id);
            // TODO put the updated worker back in the system
            // TODO add the new outgoing messages to the returned list
            let (_outbound, _new_worker) = worker.handle(&self.message);
            (vec![], my_system)
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
        let envelopes: Todos<MySystem> = vec![Box::new(e1), Box::new(e2)];
        let mut my_system = MySystem::new();
        for e in &envelopes {
            let (_, new_system) = e.deliver(my_system);
            my_system = new_system;
        }
        assert_eq!(2 + 2, 3);
    }
}
