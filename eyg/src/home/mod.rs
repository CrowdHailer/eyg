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
    fn handle(self, message: M) -> (Mail<S>, Self);
}

// Expose that there are Actors or no useful insights
// Home is the same as System or Environment or Rave or Workshop or Village,
// Workshop is interesting as Home could become skill/capability
pub trait Home<A, M>: Sized {
    type Worker: Handler<M, Self>;
    // I don't think this is very ergonomic Rust
    fn pop(self, address: &A) -> (Self::Worker, Self);
    fn put(self, address: A, worker: Self::Worker) -> Self;
}

impl<A, M, S> Deliverable<S> for Envelope<A, M> where S: Home<A, M> {
    fn deliver(self: Box<Self>, system: S) -> (Mail<S>, S) {
        let Envelope{address, message} = *self;
        let (worker, system) = system.pop(&address);
        let (mail, worker) = worker.handle(message);
        let system = system.put(address, worker);
        (mail, system)
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
            fn handle(self, _: ()) -> (Mail<S>, Self) {
                (vec![], Counter(self.0 + 1))
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
        assert_eq!(2,3)
    }
}