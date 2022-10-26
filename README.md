# Web Extensions

A Rust library that provides
[WebExtension API](https://developer.chrome.com/docs/extensions/reference/)
[WASM](https://en.wikipedia.org/wiki/WebAssembly) bindings.

This crate expresses a high level wrapper.
For a low level access there is the
[`web-extensions-sys`](https://github.com/web-extensions-rs/web-extensions-sys)
crate.

## Compatibility

This library is currently only compatible with Chrome based browsers
with [Manifest V3](https://developer.chrome.com/docs/extensions/mv3/intro/).

Once MV3 is supported by FireFox, we need to check how we can
handle it.
