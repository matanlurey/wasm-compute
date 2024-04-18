# Fastly

To learn more about and experince [Fastly][], particularly their
[Compute][wasm-compute] platform.

[fastly]: https://www.fastly.com/
[wasm-compute]: https://www.fastly.com/products/compute

<!--
Emoji reference area.

Green circle:  🟢
Yellow circle: 🟡
Red circle:    🔴
-->

## Requirements

This repository was only setup and tested on an ARM64 Mac running macOS Sonoma (14.3).

Install [homebrew](https://brew.sh/), and then run:

```shell
brew bundle install
rustup target add wasm32-wasi
```

If this is your first time using Fastly, you will need to [create an account](https://www.fastly.com/signup/).

## Resources

Links to external resources about Fastly and WebAssembly.

- [Fastly can teach you about the Wasm future in just 6 talks](https://www.fastly.com/blog/fastly-can-teach-you-about-the-wasm-future-in-just-6-talks)
- [Fastly labs](https://www.fastly.com/documentation/developers/labs/) and specifically, [Starter kits in Rust](https://www.fastly.com/documentation/solutions/starters/rust/)
- [GitHub Actions for Compute](https://github.com/fastly/compute-actions)
- [Wasmtime: A secure and fast runtime for WebAssembly](https://wasmtime.dev/)
- [Key Value Stores | Fastly](https://www.fastly.com/products/kv-store)
- [Unofficial SDKs on the Compute Platform](https://www.fastly.com/documentation/guides/compute/custom/)

## Notes

My own notes on Fastly and WebAssembly with links to primary sources.

- 🟢 [What is a WebAssembly Component (And Why)](notes/what-is-a-webassembly-component.md)
- 🔴 [Javascript Toolchain for WebAssembly Components](notes/js-toolchain-for-wasm-components.md)
- 🔴 [The WASI OS - Isolation with Communication, Wasm style](notes/wasi-os-isolation-with-communication.md)
- 🔴 [Security and Correctness in Wasmtime](notes/security-and-correctness-in-wasmtime.md)
- 🔴 [Machine Learning in Fastly's Compute](notes/machine-learning-fastly-compute.md)
