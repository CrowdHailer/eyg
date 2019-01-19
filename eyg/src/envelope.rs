//! foo
//!

// Instead of mailbox.id consider channel.partition. address
// address or partition id
pub struct Envelope<A, M> {
    pub address: A,
    pub message: M
}

// Everything is based around handling a message type,
// it is important that more than one type of message can exist in the system.
// For example a Counter might have a message type that is an enum of Increment and Reset

// Every envelope in the system has to get marked as deliverable within a given system.
// NOTE It might be possible to have a default implementation that creates a deliver<M> method that calls system.get
// So that each envelope can trivially implement the delivery protocol
// NOTE It's probably more idiomatic to mutate the system and return mail/todos but sort after first version
// NOTE rename system -> cohort???
pub trait Deliverable<S> {
    fn deliver(self: Box<Self>, system: S) -> (Mail<S>, S);
}

// A list of any of the messages that can be delivered in the system
// This call is dynamic so that new message types can be added without having to update existing code

pub type Mail<S> = Vec<Box<dyn Deliverable<S>>>;

// Think it should be possible to parameterise further by S for System using a Home trait
// Worker can have an associated id type, then only worker passed
// A static worker could wrap a box for a dynamic worker

use std::collections::HashMap;

// impl<A, M, W> Deliverable<crate::system::GeneralSystem> for crate::envelope::Envelope<A, M>
//     where W: crate::worker::Worker<M, crate::system::GeneralSystem> + 'static,
//     A: typemap::Key<Value=HashMap<A, W>> + Eq + std::hash::Hash {
//     fn deliver(self: Box<Self>, mut system: crate::system::GeneralSystem) -> (Mail<crate::system::GeneralSystem>, crate::system::GeneralSystem) {
//         let mut workers = system.states.remove::<A>().unwrap_or(HashMap::new());
//         let worker = workers.remove(&self.address).unwrap_or_else(|| W::new());
//         let (out, new_worker) = worker.handle(self.message);
//         workers.insert(self.address, new_worker);
//         system.states.insert::<A>(workers);
//         (out, system)
//     }
// }
