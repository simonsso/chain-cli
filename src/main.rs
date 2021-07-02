use keyring::AccountKeyring;
use sp_core::{crypto::Pair, U256};
// use std::io;
// use std::io::prelude::*;
use std::io::BufRead;
use substrate_api_client::{compose_extrinsic, node_metadata, Api, UncheckedExtrinsicV4, XtStatus};

/// msgcli
///

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("msgcli")
        .author("Fredrik Simonsson")
        .version("0.0.1")
        .arg(
            clap::Arg::with_name("hello")
                .long("hello")
                .help("Submit a predefined message to the chain")
                .takes_value(false),
        )
        .arg(
            clap::Arg::with_name("list")
                .long("list")
                .help("List all messages on chain")
                .takes_value(false),
        )
        .get_matches();

    let url: String = "ws://127.0.0.1:9944".into();
    let from = AccountKeyring::Alice.pair();
    let api = Api::new(url).map(|api| api.set_signer(from)).unwrap();

    // set the recipient
    // let to = AccountKeyring::Bob.to_account_id();

    // let mut next:Option<String> = Some("Hello")
    if matches.is_present("list") {
        ///! Very simple example that shows how to subscribe to events.
        use std::sync::mpsc::channel;

        // use codec::Decode;
        // use log::{debug, error};
        // use sp_core::sr25519;
        use sp_core::H256 as Hash;

        println!("Subscribe to events");
        let (events_in, events_out) = channel();
        api.subscribe_events(events_in).unwrap();

        loop {
            let event_str = events_out.recv().unwrap();

            let _unhex = Vec::from_hex(event_str).unwrap();
            let mut _er_enc = _unhex.as_slice();
            let _events = Vec::<system::EventRecord<Event, Hash>>::decode(&mut _er_enc);
            match _events {
                Ok(evts) => {
                    for evr in &evts {
                        println!("decoded: {:?} {:?}", evr.phase, evr.event);
                        match &evr.event {
                            Event::pallet_balances(be) => {
                                println!(">>>>>>>>>> balances event: {:?}", be);
                                match &be {
                                    balances::Event::Transfer(transactor, dest, value) => {
                                        println!("Transactor: {:?}", transactor);
                                        println!("Destination: {:?}", dest);
                                        println!("Value: {:?}", value);
                                        return;
                                    }
                                    _ => {
                                        debug!("ignoring unsupported balances event");
                                    }
                                }
                            }
                            _ => debug!("ignoring unsupported module event: {:?}", evr.event),
                        }
                    }
                }
                Err(_) => error!("couldn't decode event record list"),
            }
        }
    }

    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();

    if matches.is_present("hello") {
        #[allow(clippy::redundant_clone)]
        let xt: UncheckedExtrinsicV4<_> =
            compose_extrinsic!(api.clone(), "MSGModule", "do_something", "Hello Chain!!");

        println!("[+] Composed Extrinsic:\n {:?}\n", xt);

        // send and watch extrinsic until InBlock
        let tx_hash = api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)?;
        if let Some(tx_hash) = tx_hash {
            println!("[+] Transaction got included. Hash: {:?}", tx_hash);
        }
        let fetchstorage = api.get_storage_value::<Vec<u8>>("MSGModule", "Something", None);

        match fetchstorage {
            Ok(Some(ans)) => {
                println!("Something: {:?}", ans);
                if let Ok(s) = String::from_utf8(ans) {
                    println!("{}", s);
                }
            }
            Ok(_) => {
                println!("No value returned");
            }
            Err(e) => {
                println!("Error from api {:?}", e);
            }
        }
    } else {
        while let Some(Ok(s1)) = it.next() {
            #[allow(clippy::redundant_clone)]
            let xt: UncheckedExtrinsicV4<_> =
                compose_extrinsic!(api.clone(), "MSGModule", "do_something", s1);

            println!("[+] Composed Extrinsic:\n {:?}\n", xt);

            // send and watch extrinsic until InBlock
            let tx_hash = api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)?;
            if let Some(tx_hash) = tx_hash {
                println!("[+] Transaction got included. Hash: {:?}", tx_hash);
            }
        }
    }
    if matches.is_present("fetch") {
        let fetchstorage = api.get_storage_value::<U256>("MSGModule", "Something", None);

        match fetchstorage {
            Ok(Some(ans)) => {
                println!("Something: {:?}", ans);
                println!("Something: {:x}", ans);
            }
            Ok(_) => {
                println!("No value returned");
            }
            Err(e) => {
                println!("Error from api {:?}", e);
            }
        }
    }

    Ok(())
}
