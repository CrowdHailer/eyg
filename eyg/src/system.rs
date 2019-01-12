extern crate typemap;

// NOTE can edit system but only before passing to runtime
// Runtime takes ownership of system when starting
pub struct GeneralSystem {
    pub states: typemap::TypeMap
}

// Different to eyg GeneralSystem because that ties id's to mailbox
impl GeneralSystem {
    pub fn new() -> Self {
        GeneralSystem{states: typemap::TypeMap::new()}
    }
    // pub fn insert<W: Worker<M, Self> + 'static + std::fmt::Debug, M: typemap::Key<Value=HashMap<WID, W>>>(mut self, id: WID, worker: W) -> Self {
    //     let mut workers = self.states.remove::<M>().unwrap_or(HashMap::new());
    //     // println!("{:?}", workers);
    //     workers.insert(id, worker);
    //     self.states.insert::<M>(workers);
    //     self
    // }
}
