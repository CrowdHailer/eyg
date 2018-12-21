extern crate eyg;
extern crate typemap;
mod game;
mod bot;

fn main() {
    let system = eyg::GenSystem::new();
    let bot_1 = bot::State{will_play: game::Hand::Rock};
    let bot_2 = bot::State{will_play: game::Hand::Paper};

    let system = system.insert(bot::Id(1), bot_1);
    let system = system.insert(bot::Id(2), bot_2);

    let (pass_1, pass_2) = game::passes();
    let invite_1: eyg::Envelope<bot::Mailbox> = eyg::Envelope{
        id: bot::Id(1),
        message: bot::Message::Invite(pass_1)
    };
    let invite_2: eyg::Envelope<bot::Mailbox> = eyg::Envelope{
        id: bot::Id(2),
        message: bot::Message::Invite(pass_2)
    };

    let runtime = eyg::Runtime(system);
    let envelopes: eyg::Todos<eyg::GenSystem> = vec![Box::new(invite_1), Box::new(invite_2)];
    let runtime = runtime.dispatch(envelopes);

    // TODO change to player id
    // let x = pass_1.play(2, game::Hand::Rock);
    // println!("{:?}", x);
    // let x = pass_2.play(2, game::Hand::Rock);
    // println!("{:?}", x);
    println!("Hello, world!");
}
