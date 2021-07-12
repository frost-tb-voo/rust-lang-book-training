# rust-lang-book-training
Summary for a reading group of [The Rust Programming Language](https://doc.rust-lang.org/book/).

[For Japanese](
https://doc.rust-jp.rs/book-ja/)

### Build for Raspberry Pi 4 Model B

```bash
sudo apt-get install gcc-arm-linux-gnueabihf

rustup target add arm-unknown-linux-gnueabihf

cd your_workspace_with_Cargo_toml

cat <<EOF > .cargo/config
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF

cargo build --target=arm-unknown-linux-gnueabihf -p your_project --release

cp target/arm-unknown-linux-gnueabihf/release/your_program dest_dir
```

### Links

- [A good example for cross compiling](https://qiita.com/mutuya/items/f00a5b99a3f047dc3cb3), in Japanese
