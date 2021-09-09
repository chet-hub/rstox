extern crate rstox;

use rstox::core::*;
use std::str::FromStr;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

mod common;


fn main() {
    let (Se, Re): (Sender<Address>, Receiver<Address>) = mpsc::channel();

    //////////////sender robot//////////////

    thread::spawn(move || {

        let mut sender = common::tox_init("");
        Se.send(sender.get_address());

        loop {
            for ev in sender.iter() {
                match ev {
                    FriendStatusMessage(index,status) =>{
                        println!("Sender FriendStatusMessage {}: {:?}", index,status);
                    },
                    FriendStatus(index,status) =>{
                        println!("Sender FriendStatus {}: {:?}", index,status);
                        if status == UserStatus::None {
                            println!("Sender packet ");
                            //Byte to prepend, according to tox.h it should be in the range [160, 191] if lossless and [200, 254] if lossy.
                            sender.send_lossless_packet(index,&[&[160], "11111".as_bytes()].concat());
                            sender.send_lossless_packet(index,&[&[161], "11111".as_bytes()].concat());
                            sender.send_lossy_packet(index,&[&[200], "22222".as_bytes()].concat());
                            sender.send_lossy_packet(index,&[&[201], "22222".as_bytes()].concat());
                        }
                    },
                    FriendRequest(cid, _) => {
                        sender.add_friend_norequest(&cid);
                    },
                    LossyPackage(index,data) =>{
                        println!("Sender LossyPackage {}: {:?}", index,data);
                    },
                    LosslessPackage(index,data) =>{
                        println!("Sender LosslessPackage {}: {:?}", index,data);
                    },
                    ev => {
                        println!("Sender -->: {:?}", ev);
                    }
                }
            }

            sender.wait();
        }
    });


    //////////////received robot//////////////

    let mut received = common::tox_init("");
    let address = &Re.recv().unwrap();
    received.add_friend(address, "HI, I'm received robot!");

    loop {
        for ev in received.iter() {
            match ev {
                FriendStatusMessage(index,status) =>{
                    println!("Sender FriendStatusMessage {}: {:?}", index,status);
                },
                FriendStatus(index,status) =>{
                    println!("Sender FriendStatus {}: {:?}", index,status);
                },
                FriendRequest(cid, _) => {
                    received.add_friend_norequest(&cid);
                },
                LossyPackage(index,data) =>{
                    println!("Received LossyPackage {}: {:?}", index,data);
                },
                LosslessPackage(index,data) =>{
                    println!("Received LosslessPackage {}: {:?}", index,data);
                },
                ev => {
                    println!("Received -->: {:?}", ev);
                }
            }
        }

        received.wait();
    }
}
