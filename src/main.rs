// use robonomics_protocol;
// use std::env;
// use async_std::task;
// use sp_core::{sr25519, crypto::{Pair}};

extern crate clap;
use clap::{Arg, App};

fn main() {
    let _matches = App::new("SDS011 Data to Substrate Driver")
        .version("0.1.0")
        .author("Vadim Manaenko <vadim.razorq@gmail.com>")
        .about("The app publishes an IPFS hash of data to Robonomics Chain")
        .arg(Arg::with_name("suri")
            .short("s")
            .takes_value(true)
            .help("Sender account seed URI"))
        .arg(Arg::with_name("remote")
            .long("remote")
            .takes_value(true)
            .help("Substrate node WebSocket endpoint")
            .default_value("ws://localhost:9944"))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .default_value("/dev/ttyUSB0")
            .help("Specify port a sensor is connected to"))
        .get_matches();


    /*
    let suri = String::from("");
    let remote = String::from("ws://localhost:9944");
    let record = String::from("22504d322e35223a2032").into_bytes();
    let signer = sr25519::Pair::from_string(suri.as_str(), None).unwrap();


    task::block_on(
        robonomics_protocol::datalog::submit(signer, remote.as_str(), record.clone() )
    ).ok();
    */

    println!("Hello, world!");
}
