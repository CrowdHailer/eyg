//! # Gen
//!
//! Implementation of the erlang OTP abstraction layer,
//! build to be more convinient that working with raw processes.

use std::marker::PhantomData;

use crate::envelope::Mail;
use crate::worker::Worker;
use crate::system::GeneralSystem;

pub struct GenServer<T> {
    state: T,
}

use std::collections::HashMap;
pub trait GenCall<Req, Resp> {
    fn handle_call<A, W>(self, caller: Caller<A, Resp>, request: Req) -> (Mail<GeneralSystem>, Self)
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

pub struct Reply<M> {
    pub reference: i32,
    pub response: M
}

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
        let (_mail, state) = self.state.handle_call(caller, request);
        (vec![], GenServer{state})
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
        fn handle_call<A, W>(self, caller: Caller<A, i32>, request: i32) -> (Mail<GeneralSystem>, Self)
        where
            // NOTE can I remove requirement of specifying a worker exists?
            W: Worker<Reply<i32>, GeneralSystem> + 'static,
            A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash
        {
            let Caller{address, reference, ..} = caller;
            let envelope = Envelope{address, message: Reply{reference, response: request + 1}};
            (vec![Box::new(envelope)], self)
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

    #[derive(Hash, Eq, PartialEq)]
    pub struct MyServerId(pub i32);

    use std::collections::HashMap;
    impl typemap::Key for MyServerId {
        type Value = HashMap<MyServerId, MyServer>;
    }

    #[test]
    fn other_test() {
        let server = MyServer{};
        let (_out, server) = server.handle_call(Caller{address: MyServerId(5), reference: 111, _response: PhantomData}, 3);
        println!("{:?}", server);
        assert_eq!(2 + 2, 3);
    }
}
