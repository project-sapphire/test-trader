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
    let mut listener = context.socket(zmq::SUB).unwrap();
    listener.connect("tcp://localhost:1337");
    listener.set_subscribe(b"btc");
    listener.set_subscribe(b"eth");

    let mut exchange = context.socket(zmq::REQ).unwrap();
    exchange.connect("tcp://localhost:1338");

    let mut wallet = context.socket(zmq::REQ).unwrap();
    wallet.connect("tcp://localhost:1340");

    let mut rng = rand::thread_rng();

    info!("Connected; listening for ticks.");

    while let Ok(Some(update)) = prism::RateUpdate::receive(&listener, 0) {
        info!("Rates for {} on {} at {}: {:?}", update.currency, update.exchange, update.rate.timestamp, update.rate.values);

        if rng.gen_weighted_bool(5) {
            info!("Initiating exchange...");

            // Get address for destination ETH wallet
            prism::WalletRequest {
                currency: "eth".to_string(),
                query: prism::WalletQuery::Receive,
            }.send(&wallet, 0).unwrap();
            let destination = wallet.recv_string(0).unwrap().unwrap();
            debug!("Destination address: {}", destination);
            
            // Initiate exchange ()
            prism::ExchangeRequest {
                exchange: update.exchange,
                currency: "btc".to_string(),
                query: prism::ExchangeQuery::Exchange("eth".to_string(), 0.05, "".to_string(), destination),
            }.send(&exchange, 0).unwrap();
            let invoice = prism::Invoice::receive(&exchange, 0).unwrap().unwrap();
            debug!("Invoice: {:?}", invoice);

            // Pay invoice using BTC wallet
            prism::WalletRequest {
                currency: "btc".to_string(),
                query: prism::WalletQuery::Pay(invoice.amount, invoice.address),
            }.send(&wallet, 0).unwrap();
            let status = wallet.recv_string(0).unwrap().unwrap();
            info!("Invoice status: {}", status);
        }

/*
        current_rates.insert(update.currency.clone(), update.rate.clone());

        let (last_from, last_to, last_cost) = transaction_history.last().unwrap().clone();

        // if we can buy back more than what we bought for, do so!
        if last_to == update.currency && last_cost < wallets[&last_to] * update.rate.values[&last_from] {
            prism::ExchangeRequest {
                query: prism::ExchangeQuery::Exchange(last_to.clone(), last_from.clone(), wallets[&last_to], ),
                exchange: update.exchange,
                currency: update.currency
            }.send(&query, 0).unwrap();

            let invoice = prism::Invoice::receive(&query, 0).unwrap().unwrap();
            println!("Exchange invoice: {:?}", invoice);
            wallet.send_str(&invoice.address, 0).unwrap();
            println!("Paying invoice of {} {} to {}", invoice.currency.to_uppercase(), invoice.amount, invoice.address);
            let result = f64::receive(&wallet, 0).unwrap().expect("invalid payment response");
            println!("Exchange complete; received ETH {}", result);

            let v = wallets[&last_to];
            wallets.insert(last_to.clone(), v - invoice.amount);
            let v = wallets[&last_from];
            wallets.insert(last_from.clone(), v + result);
            
            // add this transaction to transaction history
            transaction_history.push((last_to, last_from, invoice.amount));

            println!("WALLET NET WORTH: BTC {}", wallets["btc"] + eth_to_btc(&current_rates, wallets["eth"]));
        }
        */

    }
}
