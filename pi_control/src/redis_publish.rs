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
