### Blink the LED on the Arduino Nano Every
This is a Rust implementation of the classic blink example for the
Arduino Nano Every.

It's largely based on
[this blog post](https://snowgoons.ro/posts/2020-11-11-compiling-rust-for-arduino-nano-every-part-one/).

## Build
```
cargo build --release
```

## Run
You need Arduino IDE. Actually only the avrdude utility is needed, but we
need the patched version that comes with Arduino IDE. Please adjust the
location in `avr-upload.sh`, then run
```
cargo run --release
```
