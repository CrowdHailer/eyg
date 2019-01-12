// For the Coordinator/Main Actor
use eyg::Mailbox;
use eyg::Todos;
use eyg::Worker;
use eyg::Deliverable;
use eyg::Home;

// For the Whole system
// TODO rename Ordered runtime
use eyg::{Runtime, Envelope};

mod counter;
mod system;

// write a main fn that counts to three then asks
// wrap in some well named top level
#[derive(Debug, Copy, Clone)]
struct Protocol;
#[derive(Debug, Copy, Clone)]
enum Message {
    Tick
}
// send four ticks but finish after three
impl Mailbox for Protocol {
    type Id = ();
    type Message = Message;
}

#[derive(Debug, Copy, Clone)]
pub struct Coordinator {
    counter_address: counter::Access
}

impl Worker<Protocol, system::System> for Coordinator {
    fn handle(self, _message: &Message) -> (Todos<system::System>, Self) {
        println!("{:?}", "received tick");

        // let envelope = Envelope{id: 1, message: counter::Message::Increment};
        (self.counter_address.inc(), self)
    }
}


// Bit weird that System is in line twice
impl Home<Protocol, system::System, Coordinator> for system::System {
    // Perhaps parameterising the get fn (rather than the impl will make it easier for a general deliverable)
    fn get(&mut self, (): <Protocol as Mailbox>::Id) -> Coordinator {
        self.coordinator
    }
    fn put(&mut self, (): <Protocol as Mailbox>::Id, worker: Coordinator) {
        self.coordinator = worker;
    }
}
// Only traits defined in the current crate can have a parameterised implementation
impl Deliverable<system::System> for Envelope<Protocol> {
    fn deliver(&self, mut system: system::System) -> (eyg::Todos<system::System>, system::System) {
        let worker: Coordinator = system.get(self.id);
        let (outbound, new_worker) = worker.handle(&self.message);
        system.put(self.id, new_worker);
        (outbound, system)
        // unimplemented!()
    }
}
// // Add ignored message type
// fn run(self) {
//     self.on_tick(|| {
//         // need to send to counter
//     }).on_tick(|| {
//
//     }).on_tick(|| {
//
//     }).stop()
// }

fn main() {
    let runtime = Runtime::new(system::System::new());
    let tick = Envelope::<Protocol>{id: (), message: Message::Tick};
    let runtime = runtime.dispatch(vec![Box::new(tick), Box::new(tick), Box::new(tick)]);
    println!("{:?}", runtime.0);
    println!("Hello, world!");
}
