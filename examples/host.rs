extern crate rstox;

use rstox::core::*;
use std::str::FromStr;


mod common;

static Me: &str = "D6F2D8F89E1A2C64ED94031B7498C1313F5D8909B909057FC6C9C165BA32A132A8B7511E0875";

fn main() {

    let mut tox = common::tox_init("");

    tox.add_friend(&Address::from_str(&Me).unwrap(), "HI, I'm host!");

    loop {
        for ev in tox.iter() {
            println!("Tox event: {:?}", ev);
            match ev {
                FriendRequest(cid, _) => {
                    tox.add_friend_norequest(&cid).unwrap();
                }
                FriendMessage(friend, _, msg) => {
                    tox.send_friend_message(friend, MessageType::Normal, &msg);
                },
                LossyPackage(friend,data) => {
                    println!("Lossy: {:?}", data);
                }
                LosslessPackage(friend,data) => {
                    println!("Lossless: {:?}", data);
                },
                ev => {}
            }
        }

        // for (friend) in tox.get_friend_list() {
        //     let response_msg: [u8; 7] = [1, 1, 1, 1, 1, 1, 1];
        //     tox.send_lossless_packet(friend, &response_msg);
        //
        //     let response_msg1: [u8; 7] = [2, 2, 2, 2, 2, 2, 2];
        //     tox.send_lossy_packet(friend, &response_msg1);
        // }

        tox.wait();
    }
}
