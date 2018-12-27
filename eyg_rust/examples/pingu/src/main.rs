extern crate typemap;
extern crate eyg;

mod sys_messages {
    pub enum Message {
        GetState
    }
}
pub mod public_messages {
    pub enum Message {
        Ping(i32)
    }
}

mod worker {
    use crate::public_messages;
    use crate::sys_messages;
    use eyg::system::GeneralSystem;
    use eyg::envelope::Mail;

    #[derive(Hash, Eq, PartialEq)]
    pub struct ID(pub i32);
    pub struct State;

    use std::collections::HashMap;
    impl typemap::Key for ID {
        type Value = HashMap<ID, State>;
    }


    impl eyg::worker::Worker<public_messages::Message, GeneralSystem> for State {
        fn new() -> Self {
            println!("{:?}", "New from Sys");
            State
        }

        fn handle(self, _message: public_messages::Message) -> (Mail<GeneralSystem>, Self) {
            (vec![], self)
        }
    }

    impl eyg::worker::Worker<sys_messages::Message, GeneralSystem> for State {
        fn new() -> Self {
            println!("{:?}", "New from public");
            State
        }
        fn handle(self, _message: sys_messages::Message) -> (Mail<GeneralSystem>, Self) {
            (vec![], self)
        }
    }
}

// mod coordinator {
//     use eyg::v2;
//     // UNIT struct because only one
//     pub struct ID;
//     pub struct State(i32);
//
//     impl State {
//         pub fn new() {
//             ping(1).then(|| {
//
//             })
//             // wait(vec![], {|| ()}).then(|pong| {
//             //
//             // })
//         }
//         fn wait(out: eyg::v2::Mail<eyg::system::GeneralSystem>, fn: Fn) {
//             unimplemented!()
//         }
//     }
// }
// #[derive(Hash, Eq, PartialEq)]
// struct OtherID(i32);
// struct OtherWorker;
// use std::collections::HashMap;
// impl typemap::Key for OtherID {
//     type Value = HashMap<OtherID, OtherWorker>;
// }
// let mut mail: eyg::v2::Mail<eyg::system::GeneralSystem> = vec![Box::new(e1),Box::new(e2)];
//     |                                                            ^^^^^^^^^^^^ the trait `eyg::worker::Worker<public_messages::Message, eyg::system::GeneralSystem>` is not implemented for `OtherWorker`
// Stops sending messges to a worker that does not implement them.

fn main() {
    use eyg::system::GeneralSystem;
    use eyg::envelope::{Envelope, Mail};

    let mut system = GeneralSystem::new();
    let e1 = Envelope{address: worker::ID(1), message: public_messages::Message::Ping(1234)};
    let e2 = Envelope{address: worker::ID(1), message: sys_messages::Message::GetState};
    let mut mail: Mail<GeneralSystem> = vec![Box::new(e1),Box::new(e2)];
    while let Some(e) = mail.pop() {
        let (_, tmp) = e.deliver(system);
        system = tmp;
    }
    println!("Hello, world!");
}
