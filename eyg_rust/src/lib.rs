// Everything is based around handling a message type,
// it is important that more than one type of message can exist in the system.
// For example a Counter might have a message type that is an enum of Increment and Reset

// This might be better called the Channel/Protocol, it is the description of how to send messages.
// Instead of mailbox.id consider channel.partition.
pub trait Mailbox: std::fmt::Debug {
    type Id: std::fmt::Debug + Copy + Eq + std::hash::Hash;
    type Message: std::fmt::Debug + Copy;
}
// An envelope is used to deliver messages to a specific mailbox.
// NOTE two envelopes with the same id but different type are for different actors.
#[derive(Copy, Clone, Debug)]
pub struct Envelope<M: Mailbox> {
    // Changing the id to a parameterised type means that Home needs to take yet another parameter, ideally it would just take a mailbox as parameter
    pub id: <M as Mailbox>::Id,
    pub message: <M as Mailbox>::Message
}

// Every envelope in the system has to get marked as deliverable within a given system.
// NOTE It might be possible to have a default implementation that creates a deliver<M> method that calls system.get
// So that each envelope can trivially implement the delivery protocol
// NOTE It's probably more idiomatic to mutate the system and return mail/todos but sort after first version
// NOTE rename system -> cohort???
pub trait Deliverable<S>: std::fmt::Debug {
    fn deliver(&self, system: S) -> (Todos<S>, S);
}

pub struct GenSystem {
    pub states: typemap::TypeMap
}

impl GenSystem {
    pub fn new() -> Self {
        GenSystem{states: typemap::TypeMap::new()}
    }
    pub fn insert<W: Worker<M, Self> + 'static + std::fmt::Debug, M: Mailbox + typemap::Key<Value=HashMap<<M as Mailbox>::Id, W>>>(mut self, id: <M as Mailbox>::Id, worker: W) -> Self {
        let mut workers = self.states.remove::<M>().unwrap_or(HashMap::new());
        // println!("{:?}", workers);
        workers.insert(id, worker);
        self.states.insert::<M>(workers);
        self
    }
}
// NOTE can edit system but only before passing to runtime
// Runtime takes ownership of system when starting

extern crate typemap;
use std::collections::HashMap;
// P for protocol/Mailbox
// S for systemm
impl<W: Worker<P, GenSystem> + 'static + std::fmt::Debug, P: Mailbox + typemap::Key<Value=HashMap<<P as Mailbox>::Id, W>>>
    Deliverable<GenSystem>
    for Envelope<P> {
    // Clean up use pop and insert on system
    fn deliver(&self, mut system: GenSystem) -> (Todos<GenSystem>, GenSystem) {
        // let worker = system.states.remove::<P>().unwrap_or_else(|| W::new());
        let mut workers = system.states.remove::<P>().unwrap_or(HashMap::new());
        let worker = workers.remove(&self.id).unwrap_or_else(|| W::new());
        let (out, new_worker) = worker.handle(&self.message);
        workers.insert(self.id, new_worker);
        system.states.insert::<P>(workers);
        (out, system)
    }
}

// A list of any of the messages that can be delivered in the system
// This call is dynamic so that new message types can be added without having to update existing code
pub type Todos<S> = Vec<Box<dyn Deliverable<S>>>;

// A worker that processes all the message of a given type with the same actor id.
pub trait Worker<M: Mailbox, S>: Sized {
    fn new() -> Self;
    fn handle(self, message: &M::Message) -> (Todos<S>, Self);
}
// The home trait specifies that a worker can be found for a certain message type
// NOTE I think by using something like typemap it should not be required to have a user defined MySystem to store actors
// It would require typemap to only hold things that implement actor,
pub trait Home<M: Mailbox, S, W: Worker<M, S>> {
    fn get(&mut self, id: M::Id) -> W;
    fn put(&mut self, id: M::Id, worker: W);
}

// implement a dispatchable trait, like home but just has dispatch

pub struct Runtime<S>(pub S);
impl<S> Runtime<S> {
    pub fn dispatch(mut self, mut envelopes: Todos<S>) -> Self {
        // TODO this works in a funny order, pops the last message and then sticks new messages on the end.
        // it should pop from one end and append messages on the other.
        // Technically no order guarantees are given so this doesn't matter but it's weird.
        while let Some(e) = envelopes.pop() {
            let mut x = e.deliver(self.0);
            envelopes.append(&mut x.0);
            self.0 = x.1;
        }
        self
    }

    pub fn new(system: S) -> Self {
        Runtime(system)
    }
}

#[cfg(test)]
mod tests {
    use crate::Envelope;
    use crate::Mailbox;
    use crate::Worker;
    use crate::Home;
    use crate::Deliverable;
    use crate::Todos;
    use crate::Runtime;

    use std::collections::HashMap;

    #[test]
    #[ignore]
    fn exhausive_janken_test() {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        struct GameId(i32);

        #[derive(Debug, Copy, Clone)]
        enum GameMove { Rock, Paper, Scissors }

        #[derive(Debug, Copy, Clone)]
        // ref to game id?
        enum GameResult { Won, Drawn, Lost }

        #[derive(Debug,Copy, Clone)]
        enum PlayerMessage {
            Invite(GameId),
            // Don't call outcome result, built in type
            Outcome(GameResult)
        }
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        struct PlayerId(i32);

        struct PlayerMailbox();
        impl Mailbox for PlayerMailbox {
            type Id = PlayerId;
            type Message = PlayerMessage;
        }

        #[derive(Debug, Copy, Clone)]
        enum GameMessage {
            Play(PlayerId, GameMove)
        }

        struct GameMailbox();
        impl Mailbox for GameMailbox {
            type Id = GameId;
            type Message = GameMessage;
        }

        // Introducing random is super difficult here
        // could have it loop around, first rock then paper,
        // send games to p1 and p2 then p2 and p3, first is a draw second not.

        // or insert players
        #[derive(Debug)]
        struct Player { will_play: GameMove }

        #[derive(Debug)]
        enum Game {
            AwaitingFirstPlayer,
            AwaitingOtherPlayer(PlayerId, GameMove),
            Concluded
        }

        #[derive(Debug)]
        struct JankenSystem {
            games: HashMap<GameId, Game>,
            players: HashMap<PlayerId, Player>,
        }

        impl Home<GameMailbox, JankenSystem, Game> for JankenSystem {
            fn get(&mut self, id: <GameMailbox as Mailbox>::Id) -> Game {
                self.games.remove(&id).unwrap_or(Game::AwaitingFirstPlayer)
            }
            fn put(&mut self, id: <GameMailbox as Mailbox>::Id, worker: Game) {
                self.games.insert(id, worker);
            }
        }

        impl Home<PlayerMailbox, JankenSystem, Player> for JankenSystem {
            fn get(&mut self, id: <PlayerMailbox as Mailbox>::Id) -> Player {
                self.players.remove(&id).unwrap()
            }
            fn put(&mut self, id: <PlayerMailbox as Mailbox>::Id, worker: Player) {
                self.players.insert(id, worker);
            }
        }

        // Maybe this should be message not mailbox?
        impl Worker<PlayerMailbox, JankenSystem> for Player {
            fn handle(self, message: &<PlayerMailbox as Mailbox>::Message) -> (Todos<JankenSystem>, Self) {
                println!("{:?}", message);
                match message {
                    PlayerMessage::Invite(game_id) =>
                        (vec![Box::new(Envelope::<GameMailbox>{id: *game_id, message: GameMessage::Play(PlayerId(1), self.will_play)})], self),
                    _message =>
                        (vec![], self)
                }
            }
        }

        impl Worker<GameMailbox, JankenSystem> for Game {
            fn handle(self, message: &<GameMailbox as Mailbox>::Message) -> (Todos<JankenSystem>, Self) {
                println!("{:?}", message);
                match message {
                    GameMessage::Play(my_player, my_move) =>
                        match self {
                            Game::AwaitingFirstPlayer =>
                                (vec![], Game::AwaitingOtherPlayer(*my_player, *my_move)),
                            Game::AwaitingOtherPlayer(other_player, GameMove::Rock) =>
                                (vec![Box::new(Envelope::<PlayerMailbox>{id: other_player, message: PlayerMessage::Outcome(GameResult::Won)})], Game::Concluded),
                            Game::AwaitingOtherPlayer(_other_player, _) =>
                                (vec![Box::new(Envelope::<PlayerMailbox>{id: *my_player, message: PlayerMessage::Outcome(GameResult::Won)})], Game::Concluded),
                            Game::Concluded =>
                                unimplemented!()
                        }
                }
            }
        }

        impl Deliverable<JankenSystem> for Envelope<GameMailbox> {
            fn deliver(&self, mut my_system: JankenSystem) -> (Todos<JankenSystem>, JankenSystem) {
                let id: <GameMailbox as Mailbox>::Id = self.id;
                let worker: Game = my_system.get(id);
                let (outbound, new_worker) = worker.handle(&self.message);
                my_system.put(id, new_worker);
                (outbound, my_system)
            }
        }
        impl Deliverable<JankenSystem> for Envelope<PlayerMailbox> {
            fn deliver(&self, mut my_system: JankenSystem) -> (Todos<JankenSystem>, JankenSystem) {
                let id: <PlayerMailbox as Mailbox>::Id = self.id;
                let worker: Player = my_system.get(id);
                let (outbound, new_worker) = worker.handle(&self.message);
                my_system.put(id, new_worker);
                (outbound, my_system)
            }
        }

        let mut players = HashMap::new();
        players.insert(PlayerId(1), Player{will_play: GameMove::Rock});
        players.insert(PlayerId(2), Player{will_play: GameMove::Paper});
        let runtime = Runtime(JankenSystem{players: players, games: HashMap::new()});
        let e1 = Envelope::<PlayerMailbox>{id: PlayerId(1), message: PlayerMessage::Invite(GameId(1))};
        let e2 = Envelope::<PlayerMailbox>{id: PlayerId(2), message: PlayerMessage::Invite(GameId(1))};
        let envelopes: Todos<JankenSystem> = vec![Box::new(e1), Box::new(e2)];
        let runtime = runtime.dispatch(envelopes);
        println!("{:?}", runtime.0);
        assert_eq!(2 + 2, 3);
        // TODO enumerate all orders
        // TODO proper wining criteria
        // Switch on deterministic or not in config
    }


    // Implement a TCP ech server
    // https://github.com/m-labs/smoltcp
    // https://www.cubrid.org/blog/understanding-tcp-ip-network-stack
    // https://www.net.in.tum.de/fileadmin/bibtex/publications/theses/2018-ixy-rust.pdf
    // Implement a Fanout and Counter and Main
    // Implement a GenCall with one use reference
    // Rock Paper Scissors
    // Write down thoughts about system level messages
    // Timeouts by sending two messages, note that you don't need to wait because switching the ordering does that
    // Good example of one of the reasons to use it.
    // Should be able to implement duplication/loss/reordering as a wrapper for handler.
    // return message in list and pass to wrapped handler for duplication, do nothing for loss.

    // If the Actor has the state of a closure you should be able to write all sorts of possible unnecessaty helpers
    // Monad.flat_map(Logger.debug("s"), {|_ok| -> do the rest})
    // Gen::{Call, Sys, Cast}
    use crate::GenSystem;
    #[test]
    fn it_works() {
        #[derive(Debug, Copy, Clone)]
        struct CounterProtocol();
        impl Mailbox for CounterProtocol {
            type Id = i32;
            type Message = Self;
        }
        #[derive(Debug, Copy, Clone)]
        struct Bar();
        // Use unit as the Id because there should only be one Bar process at a time
        impl Mailbox for Bar {
            type Id = ();
            type Message = Self;
        }
        #[derive(Debug)]
        struct FooWorker(i32);
        #[derive(Debug, Copy, Clone)]
        struct BarWorker();
        impl Worker<CounterProtocol, GenSystem> for FooWorker {
            fn new() -> Self {
                FooWorker(0)
            }
            fn handle(self, message: &CounterProtocol) -> (Todos<GenSystem>, Self) {
                println!("CounterProtocol processed {:?}", message);
                (
                    vec![Box::new(Envelope::<Bar>{id: (), message: Bar()})],
                    FooWorker(self.0 + 1)
                )
            }
        }
        impl Worker<Bar, GenSystem> for BarWorker {
            fn new() -> Self {
                BarWorker()
            }
            fn handle(self, message: &Bar) -> (Todos<GenSystem>, Self) {
                println!("Bar processed {:?}", message);
                (vec![], self)
            }

        }
        impl typemap::Key for CounterProtocol {
            type Value = HashMap<<CounterProtocol as Mailbox>::Id, FooWorker>;
        }
        impl typemap::Key for Bar {
            type Value = HashMap<<Bar as Mailbox>::Id, BarWorker>;
        }

        let runtime = Runtime(GenSystem::new());
        let e1 = Envelope::<CounterProtocol>{id: 1, message: CounterProtocol()};
        let e2 = Envelope::<Bar>{id: (), message: Bar()};
        let envelopes: Todos<GenSystem> = vec![Box::new(e1), Box::new(e2), Box::new(e1)];
        let runtime = runtime.dispatch(envelopes);
        let mut runtime = runtime.dispatch(vec![]);
        println!("{:?}", runtime.0.states.remove::<CounterProtocol>());
        // https://docs.rs/typemap/0.3.3/typemap/trait.DebugAny.html
        // trait X: core::any::Any + Mailbox {
        //
        // }
        // unsafe impl<A, B> uany::UnsafeAnyExt for X<Id=A, Message=B> {}
        // let t1 = typemap::TypeMap::<X>::custom();
        assert_eq!(2 + 2, 3);
    }
}
