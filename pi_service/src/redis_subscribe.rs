extern crate crossbeam_channel as channel;
extern crate redis;

use model::WritePwm;
use parse_rgb;
use pins::*;
use redis::{Client, PubSub};
use std::cmp::{max, min};

pub fn run(gpio_s: channel::Sender<WritePwm>) {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut pub_sub: PubSub = client.get_pubsub().unwrap();

    pub_sub.subscribe("pi_service_rgb").unwrap();
    loop {
        let msg = pub_sub.get_message();
        match msg {
            Ok(m) => {
                let p: String = m.get_payload().unwrap();
                let color = parse_rgb::hex_color(&p);
                match color {
                    Ok((_, c)) => {
                        println!(
                            "Redis receives color {}: R {} G {} B {}",
                            p, c.red, c.green, c.blue
                        );

                        gpio_s.send(WritePwm {
                            pin: RED_GPIO,
                            value: from_color(c.red),
                        });
                        gpio_s.send(WritePwm {
                            pin: GREEN_GPIO,
                            value: from_color(c.green),
                        });
                        gpio_s.send(WritePwm {
                            pin: BLUE_GPIO,
                            value: from_color(c.blue),
                        });
                    }
                    Err(_) => println!("Redis receives nonsense: {}", p),
                }
            }
            Err(_) => unimplemented!(),
        }
    }
}

// Duty cycle ranges from 0 to 100
fn from_color(color: u8) -> i32 {
    let v = (color as f32 / 255.0 * 100.0) as i32;
    max(0, min(v, 100))
}
