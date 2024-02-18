A sleep tracking tool 

On an original idea from one of my daughters.

# Compilation using xbuild

Install using `cargo install xbuild`.

- Check xbuild dependencies using `x doctor`
- Use it for android build : `x build -r --platform android --arch arm64 --format apk`
- Use it for linux build : `x build -r`

# Compilation without libc dependencies

```
# apt install musl musl-dev musl-tools
$ rustup target add x86_64-unknown-linux-musl
$ cargo build --target x86_64-unknown-linux-musl --release
```

