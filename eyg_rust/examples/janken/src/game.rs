#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Id(i32);

#[derive(Debug, Copy, Clone)]
pub enum Hand { Rock, Paper, Scissors }

#[derive(Debug, Copy, Clone)]
// ref to game id?
pub enum GameOutcome { Won, Drawn, Lost }

// Generate invite

// Important that can send back to player or Bot
// If T is a problem maybe we can use dyn
#[derive(Debug, Copy, Clone)]
pub enum Message<T: Copy + std::fmt::Debug> {
    Play(T, Hand)
}

#[derive(Debug, Copy, Clone)]
pub struct Mailbox<T>(std::marker::PhantomData<T>);
impl<T: Copy + std::fmt::Debug> eyg::Mailbox for Mailbox<T> {
    type Id = Id;
    type Message = Message<T>;
}

// TODO choose deterministic or otherwise
#[derive(Debug)]
pub enum State<T> {
    AwaitingFirstPlayer,
    AwaitingOtherPlayer(T, Hand),
    Concluded
}

// implementing home or something should be possibly simpler because this has refernece to mailbox, has reference to message
// NOTE not sure this will work properly as the T might mess up typemap key,
// Might need to use dyn in received thing
// The problem is T (type of player) might not be the same in message and state, e.g. a human playing a bot.
impl<S, T: Copy + std::fmt::Debug> eyg::Worker<Mailbox<T>, S> for State<T> where eyg::Envelope<crate::bot::Mailbox>: eyg::Deliverable<S> {
    fn new() -> Self {
        State::AwaitingFirstPlayer
    }
    fn handle(self, message: &<Mailbox<T> as eyg::Mailbox>::Message) -> (eyg::Todos<S>, Self) {
        println!("{:?}", message);
        match message {
            Message::Play(my_player, my_move) =>
                match self {
                    State::AwaitingFirstPlayer =>
                        (vec![], State::AwaitingOtherPlayer::<T>(*my_player, *my_move)),
                    State::AwaitingOtherPlayer(other_player, Hand::Rock) =>
                        (vec![Box::new(eyg::Envelope::<crate::bot::Mailbox>{id: crate::bot::Id(1), message: crate::bot::Message::Outcome(GameOutcome::Won)})], State::Concluded),
                    State::AwaitingOtherPlayer(_other_player, _) =>
                        (vec![Box::new(eyg::Envelope::<crate::bot::Mailbox>{id: crate::bot::Id(2), message: crate::bot::Message::Outcome(GameOutcome::Lost)})], State::Concluded),
                    State::Concluded =>
                        unimplemented!()
                }
        }
    }
}

// This probably belongs in a System file
use std::collections::HashMap;
impl<T: std::fmt::Debug + Copy + 'static> typemap::Key for Mailbox<T> {
    type Value = HashMap<<Mailbox<T> as eyg::Mailbox>::Id, State<T>>;
}


// Use https://doc.rust-lang.org/std/ops/trait.FnOnce.html
// to make sure that only one/two passes per game is possible
#[derive(Debug, Copy, Clone)]
pub struct Pass {
    game_id: Id
}

// Rust's super power blog post
// Passes that can be redeemed only once
// Next generating passing can be called only once
// Point into Janken dir.
// Perhaps move examples dir to top level so blog link doesn't break.
// Link to blog series in Readme, think hard about what to include and keep up to date.

// In many application you might know the caller is of a certain type so not need to parameterise on id
impl Pass {
    // redeem
    pub fn play<T: Copy + std::fmt::Debug + 'static, S>(self, player_id: T, hand: Hand) -> eyg::Todos<S> where eyg::Envelope<Mailbox<T>>: eyg::Deliverable<S> {
        vec![Box::new(eyg::Envelope::<Mailbox<T>>{id: self.game_id, message: Message::Play(player_id, hand)})]
    }
    // fn play<T: Copy + std::fmt::Debug>(self, id: T) -> Vec<Box<eyg::Envelope<Mailbox<T>>>> {
    //     unimplemented!()
    // }
}

pub fn passes() -> (Pass, Pass) {
    (Pass{game_id: Id(1)}, Pass{game_id: Id(1)})
}
