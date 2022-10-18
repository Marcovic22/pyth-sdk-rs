// example usage of reading pyth price from solana price account

use pyth_sdk_solana::load_price_feed_from_account;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};
use std::{
    thread,
    time,
};


fn main() {
    let url = "http://api.mainnet-beta.solana.com";
    // Pyth xau/usd price account on mainnet. can be found from https://pyth.network
    let key = "8y3WWjvmSmVGWVKH1rCA7VTRmuU7QbJ9axafSsBX5FcD";
    let clnt = RpcClient::new(url.to_string());
    let xau_price_key = Pubkey::from_str(key).unwrap();

    loop {
        // get price data from key
        let mut xau_price_account = clnt.get_account(&xau_price_key).unwrap();
        let xau_price_feed =
            load_price_feed_from_account(&xau_price_key, &mut xau_price_account).unwrap();

        println!(".....xau/USD.....");

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let maybe_price = xau_price_feed.get_price_no_older_than(current_time, 60);
        match maybe_price {
            Some(p) => {
                println!("price ........... {} x 10^{}", p.price, p.expo);
                println!("conf ............ {} x 10^{}", p.conf, p.expo);
            }
            None => {
                println!("price ........... unavailable");
                println!("conf ............ unavailable");
            }
        }


        let maybe_ema_price = xau_price_feed.get_ema_price_no_older_than(current_time, 60);
        match maybe_ema_price {
            Some(ema_price) => {
                println!(
                    "ema_price ....... {} x 10^{}",
                    ema_price.price, ema_price.expo
                );
                println!(
                    "ema_conf ........ {} x 10^{}",
                    ema_price.conf, ema_price.expo
                );
            }
            None => {
                println!("ema_price ....... unavailable");
                println!("ema_conf ........ unavailable");
            }
        }

        println!();

        thread::sleep(time::Duration::from_secs(1));
    }
}
