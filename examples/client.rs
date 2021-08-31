extern crate rstox;

use rstox::core::*;
use std::str::FromStr;


mod common;

static Me: &str = "D6F2D8F89E1A2C64ED94031B7498C1313F5D8909B909057FC6C9C165BA32A132A8B7511E0875";
static host: &str = "9FC3C9BF92D268E495FBBC141C56365E07B1077F56FC7838E8D4652BF002BA1540E9F98EFF28";

fn main() {

    let mut tox = common::tox_init("");

    tox.add_friend(&Address::from_str(&Me).unwrap(), "HI, I'm client!");
    tox.add_friend(&Address::from_str(&host).unwrap(), "HI, I'm client!");

    loop {
        for ev in tox.iter() {
            println!("Tox event: {:?}", ev);
            match ev {
                FriendRequest(cid, _) => {
                    tox.add_friend_norequest(&cid).unwrap();
                }
                ev => {}
            }
        }

        for (friend) in tox.get_friend_list() {
            let response_msg: [u8; 7] = [1, 1, 1, 1, 1, 1, 1];
            if let Ok(m) = tox.send_lossless_packet(friend, &response_msg) {
                println!("send_lossless_packet");
            }

            let response_msg1: [u8; 7] = [2, 2, 2, 2, 2, 2, 2];
            if let Ok(m) = tox.send_lossy_packet(friend, &response_msg1){
                println!("send_lossy_packet");
            }
        }

        tox.wait();
    }
}
