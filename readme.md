# Silence-core
Silence. _**Break the silence**_.

<p align="center">
    <img src="https://cloud-4sav6hg60-hack-club-bot.vercel.app/0image.png" alt="icon">
</p>

Silence-core provides core functions, helpers and type definitions for a voip service to work.
If you just want a quick and easy way to set up a voip service, you should use [silence](https://crates.io/crates/silence) instead as it provides a more complete (end-user friendly) approach.
This crate provides 2 main functionalities:

* Type definitions:
This crate provides Type definitions and traits in order to make the handling of packets easier.

* APIs for performing audio I/O:
The crate provides multiple ways to handle audio I/O on multiple platforms efficiently.

* APIs for receiving image input:
The crate provides ways to utilize the user's webcam to send images.

* APIs for encoding in certain codecs:
The crate provides ways to encode the raw auudio samples with [opus](https://opus-codec.org/). It also provides ways to encode raw images with the [AV1](https://en.wikipedia.org/wiki/AV1) codec.

A complete version of the documentation is available at [here](https://docs.rs/silence-core/latest).
