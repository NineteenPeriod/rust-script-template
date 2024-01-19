#### Compile for x86_64-unknown-linux-musl using release mode for speed :fire_emoji:


cargo build --target=x86_64-unknown-linux-musl --release


#### Dont forget to install the rustup target first

rustup target add x86_64-unknown-linux-musl

#### IF COMPILING ON MAC M1

brew install filosottile/musl-cross/musl-cross

#### Target for cross compile on macos
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"

export CC=x86_64-linux-musl-gcc
RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --release --target x86_64-unknown-linux-musl
