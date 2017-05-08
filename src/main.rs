extern crate zmq;

fn receive_rate(socket: &zmq::Socket) -> Result<(String, f64), String> {
    let currency = try!(try!(socket.recv_string(0).map_err(|x|"".to_string())).map_err(|x|"".to_string()));
    if currency.len() == 0 {
        return Err("".to_string());
    }

    let rate = try!(try!(socket.recv_string(0).map_err(|x|"".to_string())).map_err(|x|"".to_string()));

    Ok((currency, try!(rate.parse().map_err(|x|"couldn't parse rate".to_string()))))
}

fn main() {
    println!("Starting...");

    let mut context = zmq::Context::new();
    let mut listener = context.socket(zmq::SUB).unwrap();
    listener.connect("tcp://localhost:1337");
    listener.set_subscribe(b"");

    println!("Connected; listening for ticks.");

    while let Ok(Ok(currency)) = listener.recv_string(0) {
        println!("Rates for {}", currency);
        while let Ok((other_currency, rate)) = receive_rate(&listener) {
            println!("- {}: {}", other_currency, rate);
        }
    }
}
