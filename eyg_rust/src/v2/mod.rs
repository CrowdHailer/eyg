// This version does not type id and message type.
extern crate typemap;

use std::collections::HashMap;
pub struct Envelope<WID, M> {
    pub id: WID,
    pub message: M
}

pub trait Worker<M, S>: Sized {
    fn new() -> Self;
    fn handle(self, message: M) -> (Mail<S>, Self);
}

pub type Mail<S> = Vec<Box<dyn Deliverable<S>>>;

pub trait Deliverable<S> {
    fn deliver(self: Box<Self>, system: S) -> (Mail<S>, S);
}

pub struct GenSystem {
    pub states: typemap::TypeMap
}

// Different to eyg GenSystem because that ties id's to mailbox
impl GenSystem {
    pub fn new() -> Self {
        GenSystem{states: typemap::TypeMap::new()}
    }
    // pub fn insert<W: Worker<M, Self> + 'static + std::fmt::Debug, M: typemap::Key<Value=HashMap<WID, W>>>(mut self, id: WID, worker: W) -> Self {
    //     let mut workers = self.states.remove::<M>().unwrap_or(HashMap::new());
    //     // println!("{:?}", workers);
    //     workers.insert(id, worker);
    //     self.states.insert::<M>(workers);
    //     self
    // }
}

// Think it should be possible to parameterise further by S for System using a Home trait
// Worker can have an associated id type, then only worker passed
// A static worker could wrap a box for a dynamic worker

impl<WID, M, W> Deliverable<GenSystem> for Envelope<WID, M>
    where W: Worker<M, GenSystem> + 'static,
    WID: typemap::Key<Value=HashMap<WID, W>> + Eq + std::hash::Hash {
    fn deliver(self: Box<Self>, mut system: GenSystem) -> (Mail<GenSystem>, GenSystem) {
        // unimplemented!()
        let mut workers = system.states.remove::<WID>().unwrap_or(HashMap::new());
        let worker = workers.remove(&self.id).unwrap_or_else(|| W::new());
        let (out, new_worker) = worker.handle(self.message);
        workers.insert(self.id, new_worker);
        system.states.insert::<WID>(workers);
        (out, system)
    }
}
