extern crate typemap;
extern crate eyg;
// Experiment for more than one mailbox


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
    use eyg::v2::GenSystem;
    use eyg::v2::Mail;

    #[derive(Hash, Eq, PartialEq)]
    pub struct ID(pub i32);
    pub struct State;

    use std::collections::HashMap;
    impl typemap::Key for ID {
        type Value = HashMap<ID, State>;
    }


    impl eyg::v2::Worker<public_messages::Message, GenSystem> for State {
        fn new() -> Self {
            println!("{:?}", "New from Sys");
            State
        }

        fn handle(self, _message: public_messages::Message) -> (Mail<GenSystem>, Self) {
            (vec![], self)
        }
    }

    impl eyg::v2::Worker<sys_messages::Message, GenSystem> for State {
        fn new() -> Self {
            println!("{:?}", "New from public");
            State
        }
        fn handle(self, _message: sys_messages::Message) -> (Mail<GenSystem>, Self) {
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
//         fn wait(out: eyg::v2::Mail<eyg::v2::GenSystem>, fn: Fn) {
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
// let mut mail: eyg::v2::Mail<eyg::v2::GenSystem> = vec![Box::new(e1),Box::new(e2)];
//     |                                                            ^^^^^^^^^^^^ the trait `eyg::v2::Worker<public_messages::Message, eyg::v2::GenSystem>` is not implemented for `OtherWorker`
// Stops sending messges to a worker that does not implement them.

fn main() {
    let mut system = eyg::v2::GenSystem::new();
    let e1 = eyg::v2::Envelope{id: worker::ID(1), message: public_messages::Message::Ping(1234)};
    let e2 = eyg::v2::Envelope{id: worker::ID(1), message: sys_messages::Message::GetState};
    let mut mail: eyg::v2::Mail<eyg::v2::GenSystem> = vec![Box::new(e1),Box::new(e2)];
    while let Some(e) = mail.pop() {
        let (_, tmp) = e.deliver(system);
        system = tmp;
    }
    println!("Hello, world!");
}
