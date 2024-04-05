```
 _ _ _
| (_) |__  _ __ ___   ___  _ __   ___ _ __ ___
| | | '_ \| '_ ` _ \ / _ \| '_ \ / _ \ '__/ _ \
| | | |_) | | | | | | (_) | | | |  __/ | | (_) |
|_|_|_.__/|_| |_| |_|\___/|_| |_|\___|_|  \___/
```
---
[![Crates.io](https://img.shields.io/crates/v/libmonero.svg)](https://crates.io/crates/libmonero)
[![Docs](https://docs.rs/libmonero/badge.svg)](https://docs.rs/libmonero)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
[![Dependency Status](https://deps.rs/repo/github/monumexyz/libmonero/status.svg)](https://deps.rs/repo/github/monumexyz/libmonero)
[![Matrix](https://img.shields.io/badge/Matrix-Monume-white?logo=matrix&labelColor=grey&logoColor=white)](https://matrix.to/#/#monume:matrix.org)

> DISCLAIMER: This library is still in early development and doesn't have a stable version yet. It is not ready for production use and not cryptographically audited. We are working hard to make it as secure as possible. Please use it at your own risk.

libmonero is a powerful, batteries-included library for the Monero cryptocurrency written in Rust. It is designed to be fast, safe and easy to use.

[Why another library?](#why-another-library)

# Features

- Original (25-word) and MyMonero (13-word) seed support
- Many language support for seeds: en, eo, fr, it, ja, pt, ru, lj...
- Hexadecimal seed, private spend and view keys, address derivation
- RPC for getting data from the Monero network

And many more features... ([Roadmap](#roadmap))
 
## Usage

Add the library to your project and use the functions: \
```cargo add libmonero```

For more details, please take a look at [docs](https://docs.rs/libmonero).
If you have any questions, you can ask either at the [discussions](https://github.com/monumexyz/libmonero/discussions) or [matrix room](https://matrix.to/#/#monume:matrix.org).

## Supporting

As you may notice, we don't make any money from this project. We are working on this project for the Monero community and for the love of open-source. If you want to support us, you can donate to the following Monero address:
```
42monume4tTXqMHPDAQ3wSgUo1oN3kT9hKytkyEGYKP8g3DAWoLk2hUJ2SiwNyvC93cq3wb7VgEe7CZ4NH4dzemWFD1xd5ye
```

## Licensing

This project is licensed under the MIT License. Please take a look at [LICENSE.md](LICENSE.md) for more information.

## Roadmap

- [x] Mnemonic generation, key and address derivation
- [x] Support for MyMonero seeds
- [ ] Calculating balance for wallet
- [ ] Ability to make outgoing transactions and announcing it to network
- [ ] Support Polyseed

## Why another library?

Almost every Monero 'library' on the net is either a wrapper or a port of the official software and it's RPC. They just make interface for you to communicate with the 'real software', where all things happen. This is not the case with libmonero¹. You don't need any of the official code, which is written in C++ and is very hard to understand (some people even say wallet2 is 'hell' :D). This library is designed to be a fast, safe, and full-featured library, with support for all the features of the Monero blockchain and ecosystem.

¹: This library is written from scratch in Rust and is designed to be easy to use and understand with no need for any other software (other than Monero Daemon for blockchain data etc., NO WALLET RPC). We aim for this library to be usable even for embedded apps. That's why we are calling it 'batteries-included'.

Building this library from scratch is a huge task. We are working hard to make it as good as possible. If you want to help us, please consider contributing to this project. We are looking for people who can help us with the development, testing, and documentation. If you are interested and need any help about it, please contact community at the matrix room: [#monume:matrix.org](https://matrix.to/#/#monume:matrix.org)

TL;DR: No-bullshit, standalone, fast, safe and easy to use library for Monero did not exist, so we made one.