extern crate rstox;

use rstox::core::*;
use std::str::FromStr;
use std::fmt::Display;
use std::str;
use rstox::core::errors::BootstrapError;
use std::thread;
use std::time::Duration;


const BOOTSTRAP_NODES: [(&str, &str, &str); 9] = [
    // Impyy
    ("1D5A5F2F5D6233058BF0259B09622FB40B482E4FA0931EB8FD3AB8E7BF7DAF6F", "198.98.51.198", "33445"),
    // nurupo
    ("F404ABAA1C99A9D37D61AB54898F56793E1DEF8BD46B1038B9D822E8460FAB67", "67.215.253.85", "33445"),
    // Manolis
    ("461FA3776EF0FA655F1A05477DF1B3B614F7D6B124F7DB1DD4FE3C08B03B640F", "130.133.110.14", "33445"),
    // Busindre
    ("A179B09749AC826FF01F37A9613F6B57118AE014D4196A0E1105A98F93A54702", "205.185.116.116", "33445"),
    // ray65536
    ("8E7D0B859922EF569298B4D261A8CCB5FEA14FB91ED412A7603A585A25698832", "85.172.30.117", "33445"),
    // fluke571
    ("3CEE1F054081E7A011234883BC4FC39F661A55B73637A5AC293DDF1251D9432B", "194.249.212.109", "33445"),
    // MAH69K
    ("DA4E4ED4B697F2E9B000EEFE3A34B554ACD3F45F5C96EAEA2516DD7FF9AF7B43", "185.25.116.107", "33445"),
    // clearmartin
    ("CD133B521159541FB1D326DE9850F5E56A6C724B5B8E5EB5CD8D950408E95707", "46.101.197.175", "443"),
    // tastytea
    ("2B2137E094F743AC8BD44652C55F41DFACC502F125E99E4FE24D40537489E32F", "5.189.176.217", "5190"),
];


static BOT_NAME: &'static str = "Test bot";


pub fn tox_init(secretKey_str: &str) -> Tox {
    let mut option = ToxOptions::new();
    if let Ok(secretKey) = SecretKey::from_str(secretKey_str) {
        option = option.set_secret_key(secretKey);
    }

    let mut tox = Tox::new(option, None).unwrap();
    tox.set_name(BOT_NAME).unwrap();


    for (bootstrap_key, host, port) in BOOTSTRAP_NODES {
        match tox.bootstrap(host, port.parse().unwrap(), bootstrap_key.parse().unwrap()) {
            Ok(key) => (),
            Err(e) => print!("Bootstrap failed: {:?}", e),
        }
    }

    println!("public_key: \t{}", tox.get_address());
    println!("secret_key: \t{}", tox.get_secret_key());

    tox
}

