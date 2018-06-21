extern crate redis;

use redis::Commands;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Hello Redis {}", fetch_an_integer().unwrap());
}

fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let con = try!(client.get_connection());
    // throw away the result, just make sure it does not fail
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
    let _: () = try!(con.set("test", since_the_epoch.as_secs()));
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("test")
}
