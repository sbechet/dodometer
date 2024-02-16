A sleep tracking tool 

On an original idea from one of my daughters.

# Compilation without libc dependencies

```
# apt install musl musl-dev musl-tools
$ rustup target add x86_64-unknown-linux-musl
$ cargo build --target x86_64-unknown-linux-musl --release
```