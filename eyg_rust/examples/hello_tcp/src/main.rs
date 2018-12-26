mod tcp {
    use eyg::v2::GenSystem;
    use eyg::v2::Mail;
    struct ID;
    struct Net {

    }
    enum Message {
        Listen(std::net::SocketAddr)
    }

    impl eyg::v2::Worker<Message, GenSystem> for Net {
        fn new() -> Self {
            unimplemented!()
        }

        fn handle(self, message: Message) -> (Mail<GenSystem>, Self) {
            match message {
                Message::Listen(address) =>
                    (vec![], self)
            }
        }
    }
}

fn main() {
    let address: std::net::SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("{:?}", address);
    let listener = mio::net::TcpListener::bind(&address).unwrap();
    println!("{:?}", listener);
    let poll = mio::Poll::new().unwrap();
    let mut events = mio::Events::with_capacity(1024);
    poll.register(&listener, mio::Token(0), mio::Ready::readable(), mio::PollOpt::edge());

    let mut i = 1;
    use std::io::Read;
    use std::io::Write;
    loop {
        poll.poll(&mut events, None);
        println!("{:?}", poll);
        println!("{:?}", events);
        for event in events.iter() {
            println!("{:?}", event);
            let (mut stream, _remote) = listener.accept().unwrap();
            stream.set_keepalive(Some(std::time::Duration::from_millis(2000)));
            let mut buffer = vec![];
            stream.read_to_end(&mut buffer);
            let incoming = std::str::from_utf8(&buffer);
            stream.write_all(b"200 OK\r\ncontent-type: text/html\r\n\r\n");
            println!("{:?}", "New registering");
            println!("{:?}", i);
            let new_register = poll.register(
                &stream,
                mio::Token(i),
                mio::Ready::all(),
                mio::PollOpt::edge()
            );
            i = i + 1;
            // stream.write_bufs(&data);
            println!("{:?}", new_register);
            println!("{:?}", stream);
        }
        events.clear();
        println!("Loop end");
    }
}
