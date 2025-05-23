# esphome-device-rs

A very early stage async library for building ESPHome devices in Rust.  

At the moment, it's written using `std`, as I am using it in a project that uses
`esp-idf-hal`. I will be aiming to also add support for `no_std` using `embassy-net`, but
`std` support is likely to remain (behind a feature).

## Status

This library is in an early stage of development. It is indeed at such an early stage that
I haven't published it to crates.io yet. That will happen once things are a bit more stable.

## Scope

ESPHome has amazing integration with Home Assistant. Being a Rustacean, I wanted to 
write my firmwares in Rust. I am, however, a lot less interested in maintaining
a Home Assistant integration, and as such the obvious solution is to port the ESPHome
protocol to Rust.

This library absolutely does not aim to be a Rust port of ESPHome. It's merely a 
library that allows you to easily add ESPHome protocol support to your Rust
firmwares.

It also doesn't aim to support all features of the ESPHome protocol. Most notably,
I'm not going to be implementing any Bluetooth or Voice Assistant features.

