> rustup toolchain install nightly
> rustup component add rust-src --toolchain nightly
> cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target
> x86_64-apple-darwin --release
> rustc -vV