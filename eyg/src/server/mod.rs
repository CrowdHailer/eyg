// use super::home::*;

// Need a structure to receive out messages
// struct MyServer {
//
// }
//
// struct Connection(i32);
// // Wrap with a Connection ID
// enum TCP {
//     Packet(String),
//     Closed()
// }
//
// // Instead of subscribing with an address we could use the connection as address.
// // Need a message to say ready for one more connection
// // Things need writing in order if Writer given once then passing it would mean only one writer
// // Connection can be made without any bytes
// impl<S> Handler<(Connection, TCP), S> for MyServer {
//     fn handle(self, (connection, message): (Connection, TCP)) -> (Mail<S>, Self) {
//         match message {
//             TCP::Packet(data) =>
//                 unimplemented!(),
//             TCP::Closed() =>
//                 unimplemented!()
//         }
//     }
// // }
// // mod server {
// //     struct Server {
// //
// //     }
// //     struct Address();
// // }
// // Delegated to an enum or routes
// // Delegated to a new Address could be a part of the protocol.
// // all sorts of problems like if the has of new address being handled by a different thread
// // Data structure only water cooler for testing upgrading an address
// // Phantom Data Added to the Address
// // If A was a match/Route then we can possibly be smart about this
// #[derive(Hash)]
// struct LayeredAddress<A> {
//     id: i32,
//     _phantom: std::marker::PhantomData<A>
// }
//
// // // Option of Route
// // struct StreamUpdates { }
// // struct HomePage { }
// // // impl route
// //
// // // impl Handler<M, S> for Endpoint {
// // //
// // // }
// // // R: route
// // impl<R> Home<Server<R>, M> for Server {
// //     type Worker = Endpoint;
// // }
// //
// // // separate Actor id from address
// // impl Upgrade<Endpoint> for HomePage {
// //
// // }
// //
// // trait Upgrade<T>:  {
// //
// // }
//
// // If address is typed with M
// // single type of message
// //
//
// // pub trait Deliverable<S> {
// //     fn deliver(self: Box<Self>, system: S) -> (Mail<S>, S);
// // }
// //
// // pub type Mail<S> = Vec<Box<dyn Deliverable<S>>>;
// //
// // // Need an id to thread through entity when upgraded
// // pub trait Handler<M, S> {
// //     // type Next: Handler<M, S>;
// //     type Next;
// //     fn handle(self, message: M) -> (Mail<S>, Self::Next);
// // }
// // // I'm pretty sure it will require a dynamic step
// // // If we add a single depth wrapper then Rust will optimise it away
// // // Switch the order of message processing. i.e. a handle hits every follow on
// // impl Router {
// //     fn handle_head() {
// //         unimplemented!()
// //     }
// // }
// //
// // impl HandleHead<Route::UserPage> for Config {
// //
// // }
// //
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashMap;
//     use std::collections::hash_map::DefaultHasher;
//     use std::hash::Hash;
//
//     #[test]
//     fn server_test() {
//         let mut hasher1 = DefaultHasher::new();
//         let address: LayeredAddress<String> = LayeredAddress{id: 321, _phantom: std::marker::PhantomData};
//         address.hash(&mut hasher1);
//         println!("{:?}", hasher1);
//         let mut hasher2 = DefaultHasher::new();
//         let address: LayeredAddress<i32> = LayeredAddress{id: 321, _phantom: std::marker::PhantomData};
//         address.hash(&mut hasher2);
//         println!("{:?}", hasher2);
//         assert_eq!(2,3)
//     }
// }
