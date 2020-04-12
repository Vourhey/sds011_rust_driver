// use robonomics_protocol;
// use std::env;
// use async_std::task;
// use sp_core::{sr25519, crypto::{Pair}};

//#[macro_use]
//extern crate arrayref;
//#[macro_use]
//extern crate error_chain;

extern crate clap;
use clap::{Arg, App};


//use serialport::{SerialPort};

mod sensor;
use sensor::SDS011;
use std::thread::sleep;
use std::time::Duration;
//mod sds011;
//use sds011::Nova;
//use std::time::Duration;

//mod sensor;

fn main() {
    let matches = App::new("SDS011 Data to Substrate Driver")
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

    let device = matches.value_of("port").unwrap();
    println!("{:?}", device);

    let mut nova = SDS011::new(device);

    nova.set_work_period(5);

    loop {
        match nova.query() {
            None => println!("None returned"),
            Some(r) => println!("{:?}", r),
        }

        sleep(Duration::from_secs(30));
    }


    /*
    let mut mport = serialport::open(device).unwrap();
    mport.set_baud_rate(9600);
    mport.set_flow_control(serialport::FlowControl::None);
    mport.set_timeout(Duration::from_millis(2000));
    let mut buf = [0_u8; 10];
    loop {
        let bytes = mport.read_exact(buf.as_mut());
        println!("{:?} {:?}", buf, bytes);
    }
    */
    //let mut port = serial::open(device).map_err(|e| format!("Failed to open serial device {:?}: {}", device, e)).unwrap();
    //let mut nova = Nova::new(&mut port);

    //nova.interact().ok();

    /*
    let suri = String::from("");
    let remote = String::from("ws://localhost:9944");
    let record = String::from("22504d322e35223a2032").into_bytes();
    let signer = sr25519::Pair::from_string(suri.as_str(), None).unwrap();


    task::block_on(
        robonomics_protocol::datalog::submit(signer, remote.as_str(), record.clone() )
    ).ok();
    */

    //println!("Hello, world!");
}
