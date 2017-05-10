extern crate zmq;
extern crate prism;

use prism::{Message, ReceiveError};


fn main() {
    println!("Starting...");

    let mut context = zmq::Context::new();
    let mut listener = context.socket(zmq::SUB).unwrap();
    listener.connect("tcp://localhost:1337");
    listener.set_subscribe(b"");

    let mut query = context.socket(zmq::REQ).unwrap();
    query.connect("tcp://localhost:1338");

    println!("Connected; listening for ticks.");

    let mut i = 1;
    while let Ok(Some(update)) = prism::RateUpdate::receive(&listener, 0) {
        println!("Rates for {} on {} at {}: {:?}", update.currency, update.exchange, update.rate.timestamp, update.rate.values);

        if i % 10 == 0 {
            std::thread::sleep_ms(300);
            prism::ExchangeQuery::History.send(&query, 0);
            let response = Vec::<prism::Rate>::receive(&query, 0);
            println!("{:?}", response);
        }

        i += 1;
    }
}
