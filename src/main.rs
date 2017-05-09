extern crate zmq;
extern crate prism;

use prism::{Message, ReceiveError};


fn main() {
    println!("Starting...");

    let mut context = zmq::Context::new();
    let mut listener = context.socket(zmq::SUB).unwrap();
    listener.connect("tcp://localhost:1337");
    listener.set_subscribe(b"");

    println!("Connected; listening for ticks.");

    while let Ok(Some(update)) = prism::RateUpdate::receive(&listener) {
        println!("Rates for {} on {}: {:?}", update.currency, update.exchange, update.rate.values);
    }
}
