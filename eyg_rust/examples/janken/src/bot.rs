// TODO enumerate bot style
use crate::game;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Id(pub i32);

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Invite(game::Pass),

    // TODO normally wouldn't send whole state
    Outcome(game::GameOutcome)
}

// When win's make state happy make it an error to send another invite
#[derive(Debug)]
pub struct Mailbox();
impl eyg::Mailbox for Mailbox {
    type Id = Id;
    type Message = Message;
}

#[derive(Debug)]
pub struct State { pub will_play: game::Hand }

// This does not automatically get a deliverable implementation,
// Probably best that messages are not extensible, or use dyn.
// impl<S> eyg::Worker<Mailbox, S> for State {
//     fn handle(self, message: &Message) -> (eyg::Todos<S>, Self) {
//         match message {
//             Message::Invite(pass) =>
//                 (pass.play::<i32, S>(5, self.will_play), self)
//         }
//     }
// }
impl eyg::Worker<Mailbox, eyg::GenSystem> for State {
    fn new() -> Self {
        unimplemented!("Bots should not be initalised on demand")
    }
    fn handle(self, message: &Message) -> (eyg::Todos<eyg::GenSystem>, Self) {
        println!("{:?}", message);
        match message {
            Message::Invite(pass) =>
                (pass.play::<i32, eyg::GenSystem>(5, self.will_play), self),
            Message::Outcome(result) => {
                println!("{:?}", result);
                (vec![], self)
            }
        }
    }
}

use std::collections::HashMap;
impl typemap::Key for Mailbox {
    type Value = HashMap<<Mailbox as eyg::Mailbox>::Id, State>;
}
