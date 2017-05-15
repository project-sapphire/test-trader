#[macro_use]
extern crate log;
extern crate simplelog;
extern crate zmq;
extern crate prism;
extern crate rand;

use rand::Rng;
use prism::{Message, ReceiveError};

use std::collections::HashMap;
use std::vec::Vec;

fn main() {
    simplelog::TermLogger::init(log::LogLevelFilter::Trace,
                                simplelog::Config::default()).unwrap();
    info!("Starting...");

    let mut context = zmq::Context::new();
    let exchange = prism::exchange::Exchange::new(
        &context, "tcp://localhost:1337", "tcp://localhost:1338"
    ).unwrap();
    exchange.subscribe("btc");

    let wallet = prism::wallet::Wallet::new(
        &context, "tcp://localhost:1340"
    ).unwrap();

    info!("Connected; listening for ticks.");

    let mut rng = rand::thread_rng();
    while let Ok(Some(update)) = exchange.receive_rate_update() {
        info!("Rates for {} on {} at {}: {:?}", update.currency, update.exchange, update.rate.timestamp, update.rate.values);

        if rng.gen_weighted_bool(5) {
            info!("Initiating exchange...");

            // Get address for destination ETH wallet
            let address = wallet.receive("eth").unwrap().unwrap();
            debug!("Destination address: {}", address);
            
            // Initiate exchange
            let invoice = exchange.exchange(
                &update.exchange,
                "btc", "eth", 0.5,
                "", &address
            ).unwrap().unwrap();
            debug!("Invoice: {:?}", invoice);

            // Pay invoice using BTC wallet
            let status = wallet.pay(&invoice).unwrap().unwrap();
            info!("Invoice status: {}", status);
        }
    }
}
