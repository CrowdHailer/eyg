pub struct Envelope<A, M> {
    pub address: A,
    pub message: M
}

// It would be simpler to impl deliver on an envelope but I don't think can make dynamic
pub trait Deliverable<S> {
    fn deliver(self: Box<Self>, system: S) -> (Mail<S>, S);
}

pub type Mail<S> = Vec<Box<dyn Deliverable<S>>>;

pub trait Handler<M, S> {
    type Next;
    fn handle(self, message: M) -> (Mail<S>, Self::Next);
}

// Expose that there are Actors or no useful insights
// Home is the same as System or Environment or Rave or Workshop or Village,
// Workshop is interesting as Home could become skill/capability
// If this is a Home to these kind of workers if must be possible to insert them.
// It's up to some other step to check that if state changes stuff still works
pub trait Home<A, M>: Sized {
    type Worker: Handler<M, Self>;
    // I don't think this is very ergonomic Rust
    fn pop(self, address: &A) -> (Self::Worker, Self);
    fn put(self, address: A, worker: Self::Worker) -> Self;
}

// The specification of W here indicates this only applies for when a handler does not change the state of the worker
// impl<A, M, S, W> Deliverable<S> for Envelope<A, M>
//     where
//         S: Home<A, M, Worker=W>,
//         W: Handler<M, S, Next=S::Worker>
// {
impl<A, M, S> Deliverable<S> for Envelope<A, M>
    where
        S: Home<A, M>,
        S::Worker: Handler<M, S, Next=S::Worker>
        // W: Handler<M, S, Next=S::Worker>
{
    fn deliver(self: Box<Self>, system: S) -> (Mail<S>, S) {
        let Envelope{address, message} = *self;
        let (worker, system) = system.pop(&address);
        let (mail, worker) = worker.handle(message);
        let system = system.put(address, worker);
        (mail, system)
    }
}

mod task {
    // This might not work so well if the task needs to access config, there is no create step for the task
    pub struct Address;
    pub struct State;
    // Replace UNIT address and State with Task module something

    impl<M, S> super::Handler<M, S> for State where M: FnOnce() -> super::Mail<S> {
        fn handle(self, message: M) -> (super::Mail<S>, Self) {
            (message(), self)
        }
    }

    impl<M, S> super::Home<Address, M> for S where M: FnOnce() -> super::Mail<S> {
        type Worker = State;

        fn pop(self, _: &Address) -> (Self::Worker, Self) {
            (State, self)
        }
        fn put(self, _: Address, _: State) -> Self {
            self
        }
    }

    pub fn run<F, S>(func: F) -> Box<super::Envelope<Address, F>> where F: FnOnce() -> super::Mail<S> {
         Box::new(super::Envelope{address: Address, message: func})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn home_test() {
        #[derive(Debug)]
        pub struct Counter(i32);
        // use unit as tick for countring
        impl<S> Handler<(), S> for Counter {
            type Next = i32;
            fn handle(self, _: ()) -> (Mail<S>, i32) {
                (vec![], self.0 + 1)
            }
        }
        impl Home<String, ()> for HashMap<String, Counter> {
            type Worker = Counter;
            fn pop(mut self, address: &String) -> (Counter, Self) {
                (self.remove(address).unwrap_or(Counter(0)), self)
            }
            // How come I can use mut here but the Home trait does not specify mut
            fn put(mut self, address: String, worker: Counter) -> Self {
                self.insert(address, worker);
                self
            }
        }

        let environment = HashMap::new();
        let envelope = Envelope{address: "my_counter".to_string(), message: ()};
        let (_mail, environment) = Box::new(envelope).deliver(environment);
        println!("{:?}", environment);
        // assert_eq!(2,3)


        let e = task::run(|| {
            let messages = vec![];
            messages
        });

        let _mail: Mail<()> = vec![e];
        let e = task::run(|| {
            println!("{:?}", "Running Task");
            let messages = vec![];
            messages
        });
        struct AnySystem { }
        e.deliver(AnySystem{});
        assert_eq!(2,3);
        ()
    }
}
