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
        .get_matches();

    let url: String = "wss://internaltestnet.polkadex.trade".into();
    let from = AccountKeyring::Alice.pair();
    let api = Api::new(url).map(|api| api.set_signer(from))?;

    // set the recipient
    let to = AccountKeyring::Bob.to_account_id();

    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
     #[allow(clippy::redundant_clone)]

      // From RPC  Metadata
      //             name: PolkadexIdo,
      //calls: [
      // {
      //   name: register_investor,
      //   args: [],
      //   docs: [
      //      Registers a new investor to allow participating in funding round.,
      //     ,
      //      # Parameters,
      //     ,
      //      * `origin`: Account to be registered as Investor
      //   ]
      // },
      // {


        let xt: UncheckedExtrinsicV4<_> =
            compose_extrinsic!(api.clone(), "PolkadexIdo", "register_investor", to);

        println!("[+] Composed Extrinsic:\n {:?}\n", xt);

        // send and watch extrinsic until InBlock
        let tx_hash = api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)?;
        if let Some(tx_hash) = tx_hash {
            println!("[+] Transaction got included. Hash: {:?}", tx_hash);
        }

    Ok(())
}
