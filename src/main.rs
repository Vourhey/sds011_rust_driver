use robonomics_protocol;
// use std::env;
use async_std::task;
use sp_core::{sr25519, crypto::{Pair}};
use tempfile::NamedTempFile;
//use serde_json;

//#[macro_use]
//extern crate arrayref;
//#[macro_use]
//extern crate error_chain;

use clap::{Arg, App};

mod sensor;
use sensor::SDS011;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::io::{Write, BufReader};
//use std::fs;
use ipfs_api::IpfsClient;
use pinata_sdk::{PinataApi, PinByFile};
// use pinata_sdk::PinataApi;
// use std::fs::{OpenOptions, File};

#[tokio::main]
async fn main() {
    let matches = App::new("SDS011 Data to Substrate Driver")
        .version("0.1.0")
        .author("Vadim Manaenko <vadim.razorq@gmail.com>")
        .about("The app publishes an IPFS hash of data to Robonomics Chain")
        .arg(Arg::with_name("suri")
            .short("s")
            .takes_value(true)
            .required(true)
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
        .arg(Arg::with_name("pinata_api")
            .short("pa")
            .takes_value(true)
            .help("Pinata API Key"))
        .arg(Arg::with_name("pinata_secret")
            .short("ps")
            .takes_value(true)
            .help("Pinata Secret API Key"))
        .arg(Arg::with_name("dump_interval")
            .short("d")
            .long("dump")
            .takes_value(true)
            .default_value("60")
            .help("Interval between dumps to blockchain in minutes"))
        .arg(Arg::with_name("work_period")
            .short("w")
            .long("work")
            .takes_value(true)
            .default_value("5")
            .help("Work period in minutes"))
        .get_matches();

    let ipfs_client = IpfsClient::default();

    let device = matches.value_of("port").unwrap();

    let mut nova = SDS011::new(device);

    let work_period_str = matches.value_of("work_period").unwrap();
    let work_period = work_period_str.parse::<u8>().unwrap();
    nova.set_work_period(work_period);

    let mut time_past = Instant::now();
    let dump_period = matches.value_of("dump_interval").unwrap().parse::<u32>().unwrap();
    let dump_interval = Duration::from_secs((dump_period * 60) as u64);

    let suri = String::from(matches.value_of("suri").unwrap());
    let remote = String::from(matches.value_of("remote").unwrap());

    let mut output_file = NamedTempFile::new().unwrap();

    let pinata = if matches.is_present("pinata_api") && matches.is_present("pinata_secret") {
        let pinata_api = matches.value_of("pinata_api").unwrap();
        let pinata_secret = matches.value_of("pinata_secret").unwrap();
        Some(PinataApi::new(pinata_api, pinata_secret).unwrap())
    } else {
        None
    };

    // Write header
    writeln!(output_file, "TIMESTAMP,PM25,PM10");
    println!("{:?}", output_file.path().to_path_buf());

    loop {
        let message = nova.query().unwrap();
        println!("{:?} {:?}", Instant::now(), message);
        writeln!(output_file, "{}", message.to_csv());
        output_file.flush();

        sleep(Duration::from_secs(work_period as u64 * 60));

        if time_past.elapsed() > dump_interval {
            // TODO
            // Dump to blockchain
            let file = BufReader::new(output_file.reopen().unwrap());

            let ipfs_hash = match ipfs_client.add(file).await {
                Ok(res) => {
                    println!("{}", res.hash);
                    res.hash
                },
                Err(_e) => String::from("error adding file")
            };

            /*
            if pinata.is_some() {
                pinata.unwrap().pin_file(PinByFile::new(output_file.path().to_str().unwrap()));
            }*/

            let record = ipfs_hash.into_bytes();
            let signer = sr25519::Pair::from_string(suri.as_str(), None).unwrap();

            println!("Dumping to substrate");
            task::block_on(
                robonomics_protocol::datalog::submit(signer, remote.as_str(), record.clone() )
            ).ok();

            output_file = NamedTempFile::new().unwrap();
            // Write header
            writeln!(output_file, "TIMESTAMP,PM25,PM10");
            time_past = Instant::now();
        }
    }
}
