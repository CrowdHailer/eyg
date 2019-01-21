//! # Eyg
//!
//! Message-passing framework Rust programs.

pub mod gen;
pub mod home;
pub mod server;
pub mod envelope;
pub mod worker;
pub mod system;
pub mod runtime;

#[cfg(test)]
mod tests {
    #[test]
    fn counter_test() {
        pub struct CounterState(i32);
        use crate::worker::Worker;
        use crate::envelope::Mail;

        pub enum CounterMessage {
            Increment,
        }

        impl<S> Worker<CounterMessage, S> for CounterState {
            fn new() -> Self {
                CounterState(0)
            }

            fn handle(self, _message: CounterMessage) -> (Mail<S>, Self) {
                (vec![], CounterState(self.0 + 1))
            }
        }

        #[derive(Hash, Eq, PartialEq)]
        pub struct CounterAddress(i32);

        use std::collections::HashMap;

        impl typemap::Key for CounterAddress {
            type Value = HashMap<CounterAddress, CounterState>;
        }

        use crate::envelope::Envelope;
        use crate::system::GeneralSystem;
        use crate::runtime::OrderedRuntime;

        let system = GeneralSystem::new();

        let envelope = Envelope{
            address: CounterAddress(1),
            message: CounterMessage::Increment
        };

        // Send second
        let mail: Mail<GeneralSystem> = vec![Box::new(envelope)];

        let mut runtime = OrderedRuntime::new(system);
        runtime = runtime.dispatch(mail);
        println!("{:?}", runtime.system.states.remove::<CounterAddress>().unwrap().get(&CounterAddress(1)).unwrap().0);
        assert_eq!(2 + 2, 3);
    }
//
//     #[test]
//     fn exhausive_janken_test() {
//         #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
//         struct GameId(i32);
//
//         #[derive(Debug, Copy, Clone)]
//         enum GameMove { Rock, Paper, Scissors }
//
//         #[derive(Debug, Copy, Clone)]
//         // ref to game id?
//         enum GameResult { Won, Drawn, Lost }
//
//         #[derive(Debug,Copy, Clone)]
//         enum PlayerMessage {
//             Invite(GameId),
//             // Don't call outcome result, built in type
//             Outcome(GameResult)
//         }
//         #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
//         struct PlayerId(i32);
//
//         struct PlayerMailbox();
//         impl Mailbox for PlayerMailbox {
//             type Id = PlayerId;
//             type Message = PlayerMessage;
//         }
//
//         #[derive(Debug, Copy, Clone)]
//         enum GameMessage {
//             Play(PlayerId, GameMove)
//         }
//
//         struct GameMailbox();
//         impl Mailbox for GameMailbox {
//             type Id = GameId;
//             type Message = GameMessage;
//         }
//
//         // Introducing random is super difficult here
//         // could have it loop around, first rock then paper,
//         // send games to p1 and p2 then p2 and p3, first is a draw second not.
//
//         // or insert players
//         #[derive(Debug)]
//         struct Player { will_play: GameMove }
//
//         #[derive(Debug)]
//         enum Game {
//             AwaitingFirstPlayer,
//             AwaitingOtherPlayer(PlayerId, GameMove),
//             Concluded
//         }
//
//         #[derive(Debug)]
//         struct JankenSystem {
//             games: HashMap<GameId, Game>,
//             players: HashMap<PlayerId, Player>,
//         }
//
//         impl Home<GameMailbox, JankenSystem, Game> for JankenSystem {
//             fn get(&mut self, id: <GameMailbox as Mailbox>::Id) -> Game {
//                 self.games.remove(&id).unwrap_or(Game::AwaitingFirstPlayer)
//             }
//             fn put(&mut self, id: <GameMailbox as Mailbox>::Id, worker: Game) {
//                 self.games.insert(id, worker);
//             }
//         }
//
//         impl Home<PlayerMailbox, JankenSystem, Player> for JankenSystem {
//             fn get(&mut self, id: <PlayerMailbox as Mailbox>::Id) -> Player {
//                 self.players.remove(&id).unwrap()
//             }
//             fn put(&mut self, id: <PlayerMailbox as Mailbox>::Id, worker: Player) {
//                 self.players.insert(id, worker);
//             }
//         }
//
//         // Maybe this should be message not mailbox?
//         impl Worker<PlayerMailbox, JankenSystem> for Player {
//             fn handle(self, message: &<PlayerMailbox as Mailbox>::Message) -> (Todos<JankenSystem>, Self) {
//                 println!("{:?}", message);
//                 match message {
//                     PlayerMessage::Invite(game_id) =>
//                         (vec![Box::new(Envelope::<GameMailbox>{id: *game_id, message: GameMessage::Play(PlayerId(1), self.will_play)})], self),
//                     _message =>
//                         (vec![], self)
//                 }
//             }
//         }
//
//         impl Worker<GameMailbox, JankenSystem> for Game {
//             fn handle(self, message: &<GameMailbox as Mailbox>::Message) -> (Todos<JankenSystem>, Self) {
//                 println!("{:?}", message);
//                 match message {
//                     GameMessage::Play(my_player, my_move) =>
//                         match self {
//                             Game::AwaitingFirstPlayer =>
//                                 (vec![], Game::AwaitingOtherPlayer(*my_player, *my_move)),
//                             Game::AwaitingOtherPlayer(other_player, GameMove::Rock) =>
//                                 (vec![Box::new(Envelope::<PlayerMailbox>{id: other_player, message: PlayerMessage::Outcome(GameResult::Won)})], Game::Concluded),
//                             Game::AwaitingOtherPlayer(_other_player, _) =>
//                                 (vec![Box::new(Envelope::<PlayerMailbox>{id: *my_player, message: PlayerMessage::Outcome(GameResult::Won)})], Game::Concluded),
//                             Game::Concluded =>
//                                 unimplemented!()
//                         }
//                 }
//             }
//         }
//
//         impl Deliverable<JankenSystem> for Envelope<GameMailbox> {
//             fn deliver(&self, mut my_system: JankenSystem) -> (Todos<JankenSystem>, JankenSystem) {
//                 let id: <GameMailbox as Mailbox>::Id = self.id;
//                 let worker: Game = my_system.get(id);
//                 let (outbound, new_worker) = worker.handle(&self.message);
//                 my_system.put(id, new_worker);
//                 (outbound, my_system)
//             }
//         }
//         impl Deliverable<JankenSystem> for Envelope<PlayerMailbox> {
//             fn deliver(&self, mut my_system: JankenSystem) -> (Todos<JankenSystem>, JankenSystem) {
//                 let id: <PlayerMailbox as Mailbox>::Id = self.id;
//                 let worker: Player = my_system.get(id);
//                 let (outbound, new_worker) = worker.handle(&self.message);
//                 my_system.put(id, new_worker);
//                 (outbound, my_system)
//             }
//         }
//
//         let mut players = HashMap::new();
//         players.insert(PlayerId(1), Player{will_play: GameMove::Rock});
//         players.insert(PlayerId(2), Player{will_play: GameMove::Paper});
//         let runtime = Runtime(JankenSystem{players: players, games: HashMap::new()});
//         let e1 = Envelope::<PlayerMailbox>{id: PlayerId(1), message: PlayerMessage::Invite(GameId(1))};
//         let e2 = Envelope::<PlayerMailbox>{id: PlayerId(2), message: PlayerMessage::Invite(GameId(1))};
//         let envelopes: Todos<JankenSystem> = vec![Box::new(e1), Box::new(e2)];
//         let runtime = runtime.dispatch(envelopes);
//         println!("{:?}", runtime.0);
//         assert_eq!(2 + 2, 3);
//         // TODO enumerate all orders
//         // TODO proper wining criteria
//         // Switch on deterministic or not in config
//     }
//
//
//     // Implement a TCP ech server
//     // https://github.com/m-labs/smoltcp
//     // https://www.cubrid.org/blog/understanding-tcp-ip-network-stack
//     // https://www.net.in.tum.de/fileadmin/bibtex/publications/theses/2018-ixy-rust.pdf
//     // Implement a Fanout and Counter and Main
//     // Implement a GenCall with one use reference
//     // Rock Paper Scissors
//     // Write down thoughts about system level messages
//     // Timeouts by sending two messages, note that you don't need to wait because switching the ordering does that
//     // Good example of one of the reasons to use it.
//     // Should be able to implement duplication/loss/reordering as a wrapper for handler.
//     // return message in list and pass to wrapped handler for duplication, do nothing for loss.
//
//     // If the Actor has the state of a closure you should be able to write all sorts of possible unnecessaty helpers
//     // Monad.flat_map(Logger.debug("s"), {|_ok| -> do the rest})
//     // Gen::{Call, Sys, Cast}
//     use crate::GenSystem;
//     #[test]
//     fn it_works() {
//         #[derive(Debug, Copy, Clone)]
//         struct CounterProtocol();
//         impl Mailbox for CounterProtocol {
//             type Id = i32;
//             type Message = Self;
//         }
//         #[derive(Debug, Copy, Clone)]
//         struct Bar();
//         // Use unit as the Id because there should only be one Bar process at a time
//         impl Mailbox for Bar {
//             type Id = ();
//             type Message = Self;
//         }
//         #[derive(Debug)]
//         struct FooWorker(i32);
//         #[derive(Debug, Copy, Clone)]
//         struct BarWorker();
//         impl Worker<CounterProtocol, GenSystem> for FooWorker {
//             fn new() -> Self {
//                 FooWorker(0)
//             }
//             fn handle(self, message: &CounterProtocol) -> (Todos<GenSystem>, Self) {
//                 println!("CounterProtocol processed {:?}", message);
//                 (
//                     vec![Box::new(Envelope::<Bar>{id: (), message: Bar()})],
//                     FooWorker(self.0 + 1)
//                 )
//             }
//         }
//         impl Worker<Bar, GenSystem> for BarWorker {
//             fn new() -> Self {
//                 BarWorker()
//             }
//             fn handle(self, message: &Bar) -> (Todos<GenSystem>, Self) {
//                 println!("Bar processed {:?}", message);
//                 (vec![], self)
//             }
//
//         }
//         impl typemap::Key for CounterProtocol {
//             type Value = HashMap<<CounterProtocol as Mailbox>::Id, FooWorker>;
//         }
//         impl typemap::Key for Bar {
//             type Value = HashMap<<Bar as Mailbox>::Id, BarWorker>;
//         }
//
//         let runtime = Runtime(GenSystem::new());
//         let e1 = Envelope::<CounterProtocol>{id: 1, message: CounterProtocol()};
//         let e2 = Envelope::<Bar>{id: (), message: Bar()};
//         let envelopes: Todos<GenSystem> = vec![Box::new(e1), Box::new(e2), Box::new(e1)];
//         let runtime = runtime.dispatch(envelopes);
//         let mut runtime = runtime.dispatch(vec![]);
//         println!("{:?}", runtime.0.states.remove::<CounterProtocol>());
//         // https://docs.rs/typemap/0.3.3/typemap/trait.DebugAny.html
//         // trait X: core::any::Any + Mailbox {
//         //
//         // }
//         // unsafe impl<A, B> uany::UnsafeAnyExt for X<Id=A, Message=B> {}
//         // let t1 = typemap::TypeMap::<X>::custom();
//         assert_eq!(2 + 2, 3);
//     }
}
