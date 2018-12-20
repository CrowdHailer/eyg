use std::collections::HashMap;

#[derive(Debug)]
pub struct System {
    pub counters: HashMap<<crate::counter::Protocol as eyg::Mailbox>::Id, crate::counter::Counter>,
    pub coordinator: crate::Coordinator
}

impl System {
    pub fn new() -> Self {
        Self{counters: HashMap::new(), coordinator: crate::Coordinator{counter_address: crate::counter::access(7)}}
    }
}
