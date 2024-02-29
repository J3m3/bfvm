# bfvm

> A simple [brainf\*\*\*](https://en.wikipedia.org/wiki/Brainfuck) virtual machine capable of interpretation & JIT compilation

...which is written in Rust ~~from scratch~~ with a minimal dependency!

## Getting Started

> [!NOTE]
> Make sure you installed [Cargo](https://github.com/rust-lang/cargo) in your environment. ~~Despite having few dependency, utilizing Cargo is important because this is a modern development. Am I right?~~

> [!WARNING]
> `sizeof(memory) == 32bit cell * 2^16`

1. Clone this Repository

```console
git clone --depth=1 git@github.com:J3m3/bfvm.git bfvm
cd bfvm
```

2. Run examples

```console
cargo run -r -q -- ./example/hello.bf
```

## TODO

- [x] generate compacted IR from tokens
- [x] implement interpretor
- [x] support JIT compilation for aarch64 linux
- [ ] support JIT compilation for x86_64 linux