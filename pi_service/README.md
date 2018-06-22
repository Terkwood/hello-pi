# Pi Service

A service which executes instructions against Raspberry Pi based on messages received via Redis pub/sub.

## Running this service

You need to define `REDIS_AUTH` in your environment.  This service only supports
instances of redis with authentication set.

```sh
REDIS_AUTH=my_redis_pass cargo run
```

## Caution

Always use caution when wiring Raspberry Pi, and especially when executing arbitrary scripts that you find on the internet.  This project won't work with your Raspberry Pi wiring unless you've taken extreme care to match the configuration shown here.  This project is intended as an educational example only :fire:

## Testing the redis subscription

Try sending this from within the pi, or from a remote machine:

```sh
redis-cli -p 8379 publish pi_service_rgb "#cc00ffff"
```

## Installing Redis and stunnel on Raspberry Pi

[We followed this helpful guide](https://www.digitalocean.com/community/tutorials/how-to-encrypt-traffic-to-redis-with-stunnel-on-ubuntu-16-04), with some modifications.

We installed the following:

```sh
sudo apt-get update
sudo apt-get install redis-server
sudo apt-get install stunnel4
```

### Mac OS X stunnel configs

If you install stunnel on mac:

`/usr/local/etc/stunnel/redis-server.crt` can be used to contain your redis cert generated on the raspberry pi.

`/usr/local/etc/stunnel/stunnel.conf` should contain your client-side redis stunnel config:

```sh
pid = /run/stunnel-redis.pid

[redis-client]
client = yes
accept = 127.0.0.1:8000
connect = raspberry_pi_IP_address:6379
CAfile = /usr/local/etc/stunnel/redis-server.crt
verify = 4
```

## Examples

```sh
cargo run --example redis   # Write and read the epoch time from redis
cargo run --example simple  # Basic usage of wiringpi bindings, no message passing
cargo run --example local   # Use local message passing to drive the RGB LED
```

## Acknowledgements

[rust-wiringpi](https://github.com/Ogeon/rust-wiringpi/blob/master/src/bindings.rs) was extremely helpful here.  Thank you.

## References

* http://wiringpi.com/
* https://github.com/Ogeon/rust-wiringpi/blob/master/src/bindings.rs
* https://www.admfactory.com/breathing-light-led-on-raspberry-pi-using-c/
* http://www.bristolwatch.com/rpi/pwmRpi.htm
