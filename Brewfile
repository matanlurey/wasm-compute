# https://github.com/Homebrew/homebrew-bundle#usage

# Install Rust toolchain.
# https://rust-lang.github.io/rustup/installation/index.html
#
# Note this is intentionally not "brew 'rust'", as that will conflict with the
# Rust toolchain installed by rustup, leading to confusing build error messages
# such as https://stackoverflow.com/questions/74968490.
brew 'rustup'

# Install the Fastly CLI
# https://fastly.dev/reference/cli/
tap 'fastly/tap'
brew 'fastly'

# Install nextest (alternative to "cargo test" that Fastly also requires)
brew 'cargo-nextest'
