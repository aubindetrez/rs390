# rs390
Operating System experiments on s390 platform using Rust - Staging for OS336

This project is just an experiments, making it production ready is not my intention. Use at your
own risk.

## To dos
- Try writing kernel in rust and standalone toolchain, test with bootimage

## Changelog
- 2023-03 Start the project
- 2023-03 Fail to build a simple kernel because of a linker error

## Internal plumbing
The naming of this section was inspired by
["Git Internals - Plumbing and Porcelain"](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain),
I highly recommand this reading.

- Install rustup/cargo/rustc, check [https://www.rust-lang.org/](https://www.rust-lang.org/)
- Use rust nightly toolchain, this should solved by `kernel/rust-toolchain`. If not:
`rustup override set nightly`
- Check the available targets: `rustc --print target-list | grep s390` -> 2023-03 the only target
we can use is`s390x-unknown-linux-gnu`
- A workaround is located in `kernel/.cargo/config.toml`
- Add a s390 target `rustup target add nightly-s390x-unknown-linux-gnu`
- Get the default target configuration:
`rustc +nightly -Z unstable-options --target=s390x-unknown-linux-gnu --print target-spec-json > target-s390.json`
- Edit the configuration for standalone builds
- Optional: Compile the target: `cargo build -Z build-std --target=target-s390.json`
- Download the core source: `rustup component add rust-src`
- `cargo build --target=s390x-unknown-linux-gnu` Fails:
Remove from `target-s390.json`:
```
  "linker-flavor": "ld.lld",
  "linker": "rust-lld",
```
Because it causes a core dump:
```
error: linking with `rust-lld` failed: signal: 5 (SIGTRAP) (core dumped)
[...]
= note: PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
```
Without it:
```
/usr/bin/ld: /data/projects/rs390/kernel/target/target-s390/debug/deps/kernel-75050707ac7a3a02.19ah9aj33dachcbg.rcgu.o: error adding symbols: file in wrong format
collect2: error: ld returned 1 exit status
```


