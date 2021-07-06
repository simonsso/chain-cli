use sp_core::H256;
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
                .help("Fetch all blocks from chain and list all MSG events/extrincics")
                .takes_value(false),
        )
        .get_matches();

    let url:String = String::from("ws://127.0.0.1:9944");
    let from = AccountKeyring::Alice.pair();
    let api = Api::new(url).map(|api| api.set_signer(from))?;

    // set the recipient
    // let to = AccountKeyring::Bob.to_account_id();

    if matches.is_present("list") {
        match api.get_finalized_head() {
            Ok(Some(ans)) => {
                println!("Something: {:?}", ans);
               
                let mut myblockhash:Option<sp_core::H256> = Some(ans);
                while let Ok(Some(b)) = api.get_signed_block(myblockhash) {
                    let b:node_template_runtime::SignedBlock = b; 
                    // let (header,exc) = block.deconstruct();

                    myblockhash = Some(b.block.header.parent_hash);
                }

            }
            Ok(_) => {
                println!("No value returned");
            }
            Err(e) => {
                println!("Error from api {:?}", e);
            }
        }
    }


    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();

    // let mut next:Option<String> = Some("Hello")

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
