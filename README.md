
# UEFI

## Nightly rustfmt

```sh
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
```

(Or `rustup toolchain install nightly --component rustfmt`.)

```sh
cargo +nightly fmt
```

The project uses `nightly` for code formatting.

## References

- https://docs.rs/uefi/latest/uefi/
- https://docs.rs/uefi-raw/latest/uefi_raw/
- https://rust-osdev.github.io/uefi-rs/
- https://github.com/rust-lang/rust/issues/100499
- https://docs.rs/r-efi/latest/r_efi/

## TODO

- https://doc.rust-lang.org/unstable-book/language-features/lang-items.html
- https://doc.rust-lang.org/nightly/core/attribute.no_std.html
- `-Z build-std-features=compiler-builtins-mem` ?

  ```ini
  [unstable]
  build-std-features = ["compiler-builtins-mem"]
  build-std = ["core", "compiler_builtins"]

  [build]
  target = "x86_64-unknown-uefi"
  ```
