![Greenfield](https://img.shields.io/badge/Greenfield-0fc908.svg)
[![CI](https://github.com/xavetar/PHASEXave/actions/workflows/on_tag.yaml/badge.svg)](https://github.com/xavetar/PHASEXave/actions/workflows/on_tag.yaml)
[![Deps](https://deps.rs/repo/github/xavetar/PHASEXave/status.svg)](https://deps.rs/repo/github/xavetar/PHASEXave)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# PHASEXave

![PHASEXave Logo](api/res/phasexave-header.png)

<div style="display: flex; justify-content: center; gap: 20px;">
    <a href="https://nowpayments.io/donation?api_key=NRH28QG-ABRM7CC-J7NVGXN-F8FTRS1&source=lk_donation&medium=referral" target="_blank">
        <img src="https://nowpayments.io/images/embeds/donation-button-black.svg" alt="Crypto donation button by NOWPayments" style="height: 60px !important; width: 217px !important;">
    </a>
    <a href="https://www.buymeacoffee.com/xavetar" target="_blank">
        <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important; width: 217px !important;">
    </a>
</div>

## About

The library is hosted on [crates.io](https://crates.io/crates/PHASEXave/).

## Add library

CLI:

```shell
cargo add PHASEXave
```

Cargo.toml:

```toml
[dependencies]
PHASEXave = { version = "*" }
```

## Compilation

Go to workspace and run (Apple, Unix, Windows):

```shell
cargo build --release
```

### Cross-bulid

You need to have [Docker](https://www.docker.com/products/docker-desktop/) and [Cross](https://github.com/cross-rs/cross?tab=readme-ov-file#installation):

#### Build

```shell
cross build --release --target [target]
```

Supported cross target confirmed:

```
arm-linux-androideabi
arm-unknown-linux-musleabi
arm-unknown-linux-musleabihf
armv5te-unknown-linux-musleabi
armv7-linux-androideabi
armv7-unknown-linux-musleabihf
armv7-unknown-linux-musleabi
aarch64-linux-android
aarch64-unknown-linux-gnu
aarch64-unknown-linux-musl
i586-unknown-linux-musl
i686-linux-android
i686-unknown-linux-musl
x86_64-linux-android
x86_64-unknown-linux-gnu
x86_64-unknown-linux-musl
x86_64-pc-windows-gnu
riscv64gc-unknown-linux-gnu
```

\* - See this for [Android](https://github.com/cross-rs/cross/issues/1222) (Now works with only 1.67.0).

## Notes

### Goals

The main goals of this crate are:

1) Complete independence from third-party API, libraries.
2) Independence from unix time, the counter can be replaced by any other counter without loss of accuracy (but with a couple of non-critical changes - constants). The concept of seconds in this crate is closer in meaning to the essence of the implementation. The time zone is always in unix time (seconds) if it has been set.
3) Absolute ability to calculate the calendar (its representation) for any day of the week, month and year. The only type limiting these capabilities is u128. Currently these numbers are limited to the u128 type.
4) No problems with division, the implementation is completely independent of division, except for the leap year function, but where possible, division has been completely replaced by multiplication, and possible problems with float point are eliminated, if not forever, then for hundreds/millions of years ahead.

### Other

1) The possibility of working without std is possible, but will not be implemented (maybe temporarily, maybe not at all), I can't imagine for what purposes it may be necessary to use functionality related to date and calendar in embedded systems. Besides, it is possible to overflow with huge values days of era or unix time (seconds) or leap year function, the thresholds of which are difficult to calculate, and there is no protection against it, only if you manually set a limit. This is a very skeptical idea, for embedded systems. Time (seconds) is another matter, but I don't think anyone has problems with it. In my opinion, the use of such an approach can be addressed directly [here](https://shitcode.net/).

## Comments

Refer: [See comments](comments/)

## Examples of usage

Refer: [README.md](api/README.md)

## License

PHASEXave is primarily distributed under the terms of three the Anti-Virus license and MIT license and the Apache License (Version 2.0)

See [LICENSE-ANTI-VIRUS](LICENSE-Anti-Virus) and [LICENSE-APACHE](LICENSE-Apache) and [LICENSE-MIT](LICENSE-MIT) for details.
