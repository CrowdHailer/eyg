//! # Gen
//!
//! Implementation of the erlang OTP abstraction layer,
//! build to be more convinient that working with raw processes.

use std::marker::PhantomData;

use crate::envelope::Mail;
use crate::worker::Worker;
use crate::system::GeneralSystem;

pub trait Home<A, M>: Sized {
    // NOTE Worker has two meanings here
    type Worker: Worker<M, Self>;

    fn get(self, address: A) -> (Self::Worker, Self);
}

// This doesn't contain the required information that A can handle a message of given type
impl<A, M, W> Home<A, M> for GeneralSystem
where
    A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash + 'static,
    W: Worker<M, Self> + 'static
{
    type Worker = W;

    fn get(self, _address: A) -> (W, Self) {
        unimplemented!()
    }
}


use crate::envelope::{Envelope, Deliverable};
impl<A, M, S> Deliverable<S> for Envelope<A, M>
where
    S: Home<A, M>
{
    fn deliver(self: Box<Self>, system: S) -> (Mail<S>, S) {
        let (worker, system) = system.get(self.address);
        let (out, worker) = worker.handle(self.message);
        (out, system)
    }
}

pub enum GenServer<T> {
    Running{
        state: T,
        monitors: HashMap<i32, Box<dyn crate::envelope::Deliverable<GeneralSystem>>>
        // monitors as a map of references to messages
    },
    // This state can live a very long time.
    // Ok if it does not occupy much memory.
    // The amount of memory is set by the larges sized member of the enum, to keep that small put running state in a box.
    // Alternative is to drop enum and have a separate map of process that have terminated, but that would add the ability to stop to the runtime.
    Stopped{
        error: Option<String>
    }
}

impl<T> GenServer<T> {
    fn new(state: T) -> Self {
        GenServer::Running{state, monitors: HashMap::new()}
    }
}

use std::collections::HashMap;
pub trait GenCall<Req, Resp>: Sized {
    fn handle_call<A, S>(self, caller: Caller<A, Resp>, request: Req) -> (Mail<S>, Option<Self>)
    where
        A: 'static,
        S: Home<A, Call<A, Req, Resp>> + Home<A, Reply<Resp>>;
        // W: Worker<Reply<Resp>, GeneralSystem> + 'static,
        // A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash;
}

// A: Address
pub struct Caller<A, Resp> {
    pub address: A,
    pub reference: i32,
    _response: PhantomData<Resp>
}

// Rename Call to Request and request field to data or payload
pub struct Call<A, Req, Resp> {
    caller: Caller<A, Resp>,
    request: Req
}

// Needs an or timeout field
pub struct Reply<M> {
    pub reference: i32,
    pub response: M
}

// trait WorkerFor<M>: Worker<M, GeneralSystem> + 'static { }

impl<A, S, Req, Resp, T> Worker<Call<A, Req, Resp>, S> for GenServer<T>
    where
        // W: Worker<Reply<Resp>, GeneralSystem> + 'static,
        // A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash,
        T: GenCall<Req, Resp>,
        A: 'static,
        S: Home<A, Call<A, Req, Resp>> + Home<A, Reply<Resp>>
{
    fn new() -> Self {
        unimplemented!()
    }
    fn handle(self, Call{caller, request}: Call<A, Req, Resp>) -> (Mail<S>, Self) {
        match self {
            GenServer::Running{state, monitors} =>
                {
                    // Catch a panic, requires all types to implement
                    //  consider adding a `where A: std::panic::UnwindSafe` bound
                    // let r = std::panic::catch_unwind(|| {
                        match state.handle_call::<A, S>(caller, request) {
                            (_mail, Some(state)) =>
                                (vec![], GenServer::Running{state, monitors: HashMap::new()}),
                            (_mail, None) =>
                                {
                                    println!("{:?}", "Stopping the server");
                                    (vec![], GenServer::Stopped{error: None})
                                }
                        }
                    // });
                    // match r {
                    //     Ok(value) =>
                    //         value
                    // }
                }
// .map_or_else(|e| {println!("{:?}", e); (vec![], GenServer::Stopped{error: None})}, |a| a),
            _ =>
                unimplemented!()
        }
    }
}
// I'm sure we can reuse the Call/Calling type
struct Monitor<A> {
address: A
}
struct Down {
    error: Option<String>,
    reference: i32
}
// Needs demonitor
impl<A, T, S> Worker<Monitor<A>, S> for GenServer<T>
    where
        // Replacing this with worker for doesn't work as expected here
        // W: Worker<Down, GeneralSystem> + 'static,
        // A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash,
        A: 'static,
        S: Home<A, Down>
{
    fn new() -> Self {
        unimplemented!()
    }
    fn handle(self, message: Monitor<A>) -> (Mail<S>, Self) {
        let Monitor{address} = message;
        match self {
            GenServer::Running{state, monitors} =>
                (vec![], GenServer::Running{state, monitors}),
            GenServer::Stopped{error} =>
                {
                    let message = Down{error: error.clone(), reference: 1};
                    let envelope = crate::envelope::Envelope{address, message};

                    (vec![Box::new(envelope)], GenServer::Stopped{error})
                }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::envelope::Envelope;
    #[derive(Debug)]
    pub struct MyServer {

    }

    impl GenCall<i32, i32> for MyServer {
        fn handle_call<A, S>(self, caller: Caller<A, i32>, request: i32) -> (Mail<S>, Option<Self>)
        where
            // NOTE can I remove requirement of specifying a worker exists?
            // W: Worker<Reply<i32>, GeneralSystem> + 'static,
            // A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash
            A: 'static,
            S: Home<A, Call<A, i32, i32>> + Home<A, Reply<i32>>
        {
            let Caller{address, reference, ..} = caller;
            let envelope = Envelope{address, message: Reply{reference, response: request + 1}};
            (vec![Box::new(envelope)], Some(self))
        }
    }
    // impl GenCall<String, ()> for MyServer {
    //     fn handle_call<A, W>(self, caller: Caller<A, ()>, _request: String) -> (Mail<GeneralSystem>, Option<Self>)
    //     where
    //         // NOTE can I remove requirement of specifying a worker exists?
    //         W: Worker<Reply<()>, GeneralSystem> + 'static,
    //         A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash
    //     {
    //         let Caller{address, reference, ..} = caller;
    //         let envelope = Envelope{address, message: Reply{reference, response: ()}};
    //         (vec![Box::new(envelope)], Some(self))
    //     }
    // }

    enum Only {
        One
    }
    impl GenCall<Only, ()> for MyServer {
        fn handle_call<A, S>(self, caller: Caller<A, ()>, _request: Only) -> (Mail<S>, Option<Self>)
        where
            // NOTE can I remove requirement of specifying a worker exists?
            // W: Worker<Reply<()>, GeneralSystem> + 'static,
            // A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash
            A: 'static,
            S: Home<A, Call<A, Only, ()>> + Home<A, Reply<()>>
        {
            let Caller{address, reference, ..} = caller;
            let envelope = Envelope{address, message: Reply{reference, response: ()}};
            (vec![Box::new(envelope)], None)
        }
    }

    impl<S> Worker<Reply<i32>, S> for MyServer {
        fn new() -> Self {
            unimplemented!()
        }
        fn handle(self, Reply{response, ..}: Reply<i32>) -> (Mail<S>, Self) {
            println!("{:?}", response);
            unimplemented!()
        }
    }
    impl<S> Worker<Reply<()>, S> for MyServer {
        fn new() -> Self {
            unimplemented!()
        }
        fn handle(self, Reply{response, ..}: Reply<()>) -> (Mail<S>, Self) {
            println!("{:?}", response);
            unimplemented!()
        }
    }
    impl<S> Worker<Down, S> for MyServer {
        fn new() -> Self {
            unimplemented!()
        }
        fn handle(self, Down{error, reference}: Down) -> (Mail<S>, Self) {
            println!("{:?}", reference);
            println!("{:?}", error);
            unimplemented!()
        }
    }

    #[derive(Hash, Eq, PartialEq)]
    pub struct MyServerId(pub i32);

    use std::collections::HashMap;
    impl typemap::Key for MyServerId {
        type Value = HashMap<MyServerId, MyServer>;
    }

    #[test]
    fn other_test() {
        let server_state = MyServer{};
        let server = GenServer::new(server_state);
        let call = Call{caller: Caller{address: MyServerId(5), reference: 111, _response: PhantomData}, request: 43};
        let (_out, server): (Mail<GeneralSystem>, GenServer<MyServer>) = server.handle(call);
        let call = Call{caller: Caller{address: MyServerId(5), reference: 111, _response: PhantomData}, request: Only::One};
        let (_out, server) = server.handle(call);
        let monitor = Monitor{address: MyServerId(5)};
        let (out, server) = server.handle(monitor);
        println!("Count - {:?}", out.iter().count());
        // let (_out, server) = server.handle(call);
        // let (_out, server) = server.handle(Caller{address: MyServerId(5), reference: 111, _response: PhantomData}, 21);
        // let (_out, server) = server.handle(Caller{address: MyServerId(5), reference: 111, _response: PhantomData}, Only::One);
        // println!("{:?}", server);
        assert_eq!(2 + 2, 3);
    }
}
