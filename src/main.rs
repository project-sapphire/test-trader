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

    let mut wallet = context.socket(zmq::REQ).unwrap();
    wallet.connect("tcp://localhost:1339");

    println!("Connected; listening for ticks.");

    let mut i = 1;
    while let Ok(Some(update)) = prism::RateUpdate::receive(&listener, 0) {
        println!("Rates for {} on {} at {}: {:?}", update.currency, update.exchange, update.rate.timestamp, update.rate.values);

        if i % 10 == 0 {
            std::thread::sleep_ms(300);
            prism::ExchangeRequest {
                query: prism::ExchangeQuery::Exchange("btc".to_string(), "eth".to_string(), 0.5),
                exchange: update.exchange,
                currency: update.currency
            }.send(&query, 0).unwrap();

            let invoice = prism::Invoice::receive(&query, 0).unwrap().unwrap();
            println!("Exchange invoice: {:?}", invoice);
            wallet.send_str(&invoice.address, 0).unwrap();
            println!("Paying invoice of {} {} to {}", invoice.currency.to_uppercase(), invoice.amount, invoice.address);
            let result = f64::receive(&wallet, 0).unwrap().expect("invalid payment response");
            println!("Exchange complete; received ETH {}", result);
        }

        i += 1;
    }
}
