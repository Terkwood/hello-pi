extern crate crossbeam_channel as channel;
extern crate redis;

use model::WritePwm;
use redis::{Client, PubSub};

pub fn run(gpio_s: channel::Sender<WritePwm>) {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut pub_sub: PubSub = client.get_pubsub().unwrap();

    pub_sub.subscribe("pi_service").unwrap();
    loop {
        let msg = pub_sub.get_message();
        match msg {
            Ok(m) => {
                let p: String = m.get_payload().unwrap();
                println!("Redis receives {}", p);
            }
            Err(_) => unimplemented!(),
        }
    }
}
