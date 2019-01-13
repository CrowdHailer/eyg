//! # Gen
//!
//! Implementation of the erlang OTP abstraction layer,
//! build to be more convinient that working with raw processes.

use std::marker::PhantomData;

use crate::envelope::Mail;
use crate::worker::Worker;
use crate::system::GeneralSystem;

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
    fn handle_call<A, W>(self, caller: Caller<A, Resp>, request: Req) -> (Mail<GeneralSystem>, Option<Self>)
    where
        W: Worker<Reply<Resp>, GeneralSystem> + 'static,
        A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash;
}

// A: Address
pub struct Caller<A, Resp> {
    pub address: A,
    pub reference: i32,
    _response: PhantomData<Resp>
}

// Rename Call to Request and request field to data or payload
struct Call<A, Req, Resp> {
    caller: Caller<A, Resp>,
    request: Req
}

// Needs an or timeout field
pub struct Reply<M> {
    pub reference: i32,
    pub response: M
}

trait WorkerFor<M>: Worker<M, GeneralSystem> + 'static { }

impl<A, W, Req, Resp, T> Worker<Call<A, Req, Resp>, GeneralSystem> for GenServer<T>
    where
        W: Worker<Reply<Resp>, GeneralSystem> + 'static,
        A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash,
        T: GenCall<Req, Resp>,
{
    fn new() -> Self {
        unimplemented!()
    }
    fn handle(self, Call{caller, request}: Call<A, Req, Resp>) -> (Mail<GeneralSystem>, Self) {
        match self {
            GenServer::Running{state, monitors} =>
                {
                    // Catch a panic, requires all types to implement
                    //  consider adding a `where A: std::panic::UnwindSafe` bound
                    // let r = std::panic::catch_unwind(|| {
                        match state.handle_call(caller, request) {
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
impl<A, W, T> Worker<Monitor<A>, GeneralSystem> for GenServer<T>
    where
        // Replacing this with worker for doesn't work as expected here
        W: Worker<Down, GeneralSystem> + 'static,
        A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash,
{
    fn new() -> Self {
        unimplemented!()
    }
    fn handle(self, message: Monitor<A>) -> (Mail<GeneralSystem>, Self) {
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
        fn handle_call<A, W>(self, caller: Caller<A, i32>, request: i32) -> (Mail<GeneralSystem>, Option<Self>)
        where
            // NOTE can I remove requirement of specifying a worker exists?
            W: Worker<Reply<i32>, GeneralSystem> + 'static,
            A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash
        {
            let Caller{address, reference, ..} = caller;
            let envelope = Envelope{address, message: Reply{reference, response: request + 1}};
            (vec![Box::new(envelope)], Some(self))
        }
    }
    impl GenCall<String, ()> for MyServer {
        fn handle_call<A, W>(self, caller: Caller<A, ()>, _request: String) -> (Mail<GeneralSystem>, Option<Self>)
        where
            // NOTE can I remove requirement of specifying a worker exists?
            W: Worker<Reply<()>, GeneralSystem> + 'static,
            A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash
        {
            let Caller{address, reference, ..} = caller;
            let envelope = Envelope{address, message: Reply{reference, response: ()}};
            (vec![Box::new(envelope)], Some(self))
        }
    }

    enum Only {
        One
    }
    impl GenCall<Only, ()> for MyServer {
        fn handle_call<A, W>(self, caller: Caller<A, ()>, _request: Only) -> (Mail<GeneralSystem>, Option<Self>)
        where
            // NOTE can I remove requirement of specifying a worker exists?
            W: Worker<Reply<()>, GeneralSystem> + 'static,
            A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash
        {
            let Caller{address, reference, ..} = caller;
            let envelope = Envelope{address, message: Reply{reference, response: ()}};
            (vec![Box::new(envelope)], None)
        }
    }

    impl Worker<Reply<i32>, GeneralSystem> for MyServer {
        fn new() -> Self {
            unimplemented!()
        }
        fn handle(self, Reply{response, ..}: Reply<i32>) -> (Mail<GeneralSystem>, Self) {
            println!("{:?}", response);
            unimplemented!()
        }
    }
    impl Worker<Reply<()>, GeneralSystem> for MyServer {
        fn new() -> Self {
            unimplemented!()
        }
        fn handle(self, Reply{response, ..}: Reply<()>) -> (Mail<GeneralSystem>, Self) {
            println!("{:?}", response);
            unimplemented!()
        }
    }
    impl Worker<Down, GeneralSystem> for MyServer {
        fn new() -> Self {
            unimplemented!()
        }
        fn handle(self, Down{error, reference}: Down) -> (Mail<GeneralSystem>, Self) {
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
        let (_out, server) = server.handle(call);
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
