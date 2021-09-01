# InterlockLedger test-utils

## Description

**il2-test-utils** is a *Rust* library designed to provide unit-test utilities
that can be used by other components of **InterlockLedger** written in *Rust*.

The components of this library are designed to be as easy as possible to use inside
unit tests. As such, some of the methods just panic when something goes wrong. Do
not use it in production code!

## How to use it

To use this library, just add it to your `Cargo.toml`. Visit
[crate.io](https://crates.io/crates/il2-test-utils) to get the latest "stable" releases.

If you want to use the latest version, just add the following lines into your `Cargo.toml`.

```toml
[dev-dependencies]
il2-test-utils={ git = "https://github.com/interlockledger/rust-il2-test-utils.git" }
```

## Compatibility

This library will always try to keep compatibility with older versions as changes
in its public interface may break unit-tests using it.

## Project history

- 0.1.1:
    - Initial release with metadata fixed;

## License

This library is licensed under a 3-Clause BSD license.
