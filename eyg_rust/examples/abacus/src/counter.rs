extern crate eyg;
use eyg::{Mailbox, Worker};
use eyg::{Todos};
use eyg::{Home};
use eyg::{Envelope};
use eyg::{Deliverable};

// Might be the case we don't need this and we can work for any system??
use crate::system::System;

pub struct Protocol;

// TODO a reply message that doesn't requre knowledge of main mailbox
#[derive(Debug, Copy, Clone)]
pub enum Message {
    Increment
}

impl Mailbox for Protocol {
    type Id = i32;
    type Message = Message;
}


#[derive(Debug, Copy, Clone)]
pub struct Counter {
    total: i32
}

// This worker doesn't need to know about the system,
// We are just satisfying the type checker that messages will be all right
impl<S> Worker<Protocol, S> for Counter {

    fn handle(self, _message: &Message) -> (Todos<S>, Self) {
        // Would be nice to have a helper that made create vec of box of deliverable easier
        (vec![], Self{total: self.total + 1})
    }
}

impl Deliverable<crate::system::System> for Envelope<Protocol> {
    // Forget the home trait and deliverable can call system directly
    // Home trait only useful in the case where deliverable can be implemented automatically on top
    fn deliver(&self, mut system: System) -> (eyg::Todos<System>, System) {
        let worker: Counter = system.get(self.id);
        let (outbound, new_worker) = worker.handle(&self.message);
        system.put(self.id, new_worker);
        (outbound, system)
        // unimplemented!()
    }
}
// parameterised imple might be possible in app, because you create all protocols and Worker
// Though that's not something I want to build in, because it makes a mess of extensible, for example, loggers

impl Home<Protocol, System, Counter> for System {
    // Perhaps parameterising the get fn (rather than the impl will make it easier for a general deliverable)
    fn get(&mut self, id: <Protocol as Mailbox>::Id) -> Counter {
        // Probably best to make a get counter fn on system
        self.counters.remove(&id).unwrap_or(Counter{total: 0})
    }
    fn put(&mut self, id: <Protocol as Mailbox>::Id, worker: Counter) {
        self.counters.insert(id, worker);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Access {
    address: i32
}

impl Access {
    pub fn inc(self) -> Todos<System> {
        let e = Envelope::<Protocol>{id: self.address, message: Message::Increment};
        vec![Box::new(e)]
    }
}
pub fn access(address: i32) -> Access {
    Access{address: address}
}
