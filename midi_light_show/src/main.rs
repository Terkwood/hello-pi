// SPDX-License-Identifier: MIT
use log::info;
use midi_light_show::controls::Controls;
use midi_light_show::DEFAULT_OUTPUT_DEVICE;

use std::env;
use std::sync::{Arc, Mutex};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("{}", VERSION);
    let mut args: env::Args = env::args();
    args.next();
    let mut files = vec![];
    while let Some(f) = args.next() {
        files.push(f);
    }
    let (c, _) =
        midi_light_show::controls::MsgPassingControls::create(files, DEFAULT_OUTPUT_DEVICE);
    let controls = Arc::new(Mutex::new(c));
    let pcc = controls.clone();
    let ncc = controls.clone();
    let scc = controls.clone();

    // POST /play => { .. song status .. }
    let play = warp::post().and(warp::path!("play")).map(move || {
        if let Ok(status) = pcc.lock().expect("mutex").play() {
            info!("PLAY");
            warp::reply::json(&status)
        } else {
            panic!("PLAY")
        }
    });

    // POST /stop => { .. song status .. }
    let stop = warp::post().and(warp::path!("stop")).map(move || {
        if let Ok(status) = scc.lock().expect("mutex").stop() {
            info!("STOP");
            warp::reply::json(&status)
        } else {
            panic!("STOP")
        }
    });

    // POST /next => { .. song status .. }
    let next = warp::post().and(warp::path!("next")).map(move || {
        if let Ok(status) = ncc.lock().expect("mutex").next() {
            info!("NEXT");
            warp::reply::json(&status)
        } else {
            panic!("NEXT")
        }
    });

    // POST /prev => { .. song status .. }
    let prev = warp::post().and(warp::path!("prev")).map(move || {
        if let Ok(status) = controls.lock().expect("mutex").prev() {
            info!("PREV");
            warp::reply::json(&status)
        } else {
            panic!("PREV")
        }
    });

    warp::serve(play.or(stop).or(next).or(prev))
        .run(([0, 0, 0, 0], 3030))
        .await;
}
