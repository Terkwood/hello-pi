//MIT License

//Copyright (c) 2018 Terkwood

//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

extern crate config;
extern crate crossbeam_channel as channel;
extern crate redis;

use model::WritePwm;
use parse_rgb;
use pins::*;
use redis::{Client, PubSub};
use std::cmp::{max, min};

pub fn run(gpio_s: channel::Sender<WritePwm>) {
    let rcfg = RedisConfig::new();
    let client =
        Client::open(&format!("redis://:{}@127.0.0.1:{}/", rcfg.auth, rcfg.port)[..]).unwrap();
    let mut pub_sub: PubSub = client.get_pubsub().unwrap();

    pub_sub.subscribe(rcfg.channel).unwrap();
    loop {
        let msg = pub_sub.get_message();
        match msg {
            Ok(m) => {
                let p: String = m.get_payload().unwrap();
                let color = parse_rgb::hex_color(&p);
                match color {
                    Ok((_, c)) => {
                        println!(
                            "Redis receives color {}: R {} G {} B {} A {}",
                            p, c.red, c.green, c.blue, c.alpha
                        );

                        gpio_s.send(WritePwm {
                            pin: RED_GPIO,
                            value: from_color(c.red, c.alpha),
                        });
                        gpio_s.send(WritePwm {
                            pin: GREEN_GPIO,
                            value: from_color(c.green, c.alpha),
                        });
                        gpio_s.send(WritePwm {
                            pin: BLUE_GPIO,
                            value: from_color(c.blue, c.alpha),
                        });
                    }
                    Err(_) => println!("Redis receives nonsense: {}", p),
                }
            }
            Err(_) => println!("Redis receive error"),
        }
    }
}

// Duty cycle ranges from 0 to 100
fn from_color(color: u8, alpha: u8) -> i32 {
    let v = color as f32 / 255.0 * 100.0;
    let a = alpha as f32 / 255.0 * 100.0;
    max(0, min((v * a) as i32, 100))
}

struct RedisConfig {
    auth: String,
    port: i32,
    channel: String,
}

impl RedisConfig {
    fn new() -> RedisConfig {
        let mut settings = config::Config::default();
        settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name("Settings")).unwrap()
        // Add in settings from the environment 
        .merge(config::Environment::default()).unwrap();

        RedisConfig {
            auth: settings.get::<String>("redis.auth").unwrap(),
            port: settings.get::<i32>("redis.port").unwrap_or(6379),
            channel: settings.get::<String>("redis.channel").unwrap(),
        }
    }
}
