// Copyright (c) 2018 Terkwood
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate config;
extern crate crossbeam_channel as channel;
extern crate redis;

use model::SetRGB;

use redis::{Client, Commands};

pub fn run(redis_r: channel::Receiver<SetRGB>) {
    let rcfg = RedisConfig::new();
    let client =
        Client::open(&format!("redis://:{}@127.0.0.1:{}/", rcfg.auth, rcfg.port)[..]).unwrap();
    let con = client.get_connection().unwrap();

    loop {
        match redis_r.recv() {
            Some(SetRGB { color }) => {
                let cmd = command_string(color);
                println!("redis publish pi_service_rgb {}", cmd);
                con.publish(&rcfg.channel[..], cmd).unwrap()
            }
            None => {}
        }
    }
}

/// e.g.
/// command_string([0.25827336, 0.10971709, 0.29434448, 0.0])
/// yields RGB#411B4B00
fn command_string(color: [f32; 4]) -> String {
    let rgb = to_rgb(color);
    format!(
        "RGB#{:02X}{:02X}{:02X}{:02X}",
        rgb[0], rgb[1], rgb[2], rgb[3]
    )
}

fn to_rgb(color: [f32; 4]) -> [i32; 4] {
    let mut out: [i32; 4] = [0; 4];

    for (i, elem) in color.iter().enumerate() {
        out[i] += (elem * 255.0) as i32
    }

    out
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
            port: settings.get::<i32>("redis.port").unwrap(),
            channel: settings.get::<String>("redis.channel").unwrap(),
        }
    }
}
