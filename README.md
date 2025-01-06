# bfvm

> A simple [brainf\*\*\*](https://en.wikipedia.org/wiki/Brainfuck) virtual machine capable of interpretation & JIT compilation

...which is written in Rust ~~from scratch~~ with minimal dependencies!

## Getting Started

> [!NOTE]
> Make sure you installed [Cargo](https://github.com/rust-lang/cargo) in your environment.

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

- [x] generate (something similar to) IR from tokens
- [x] implement the interpreter
- [x] support JIT compilation for aarch64 linux
- [ ] support JIT compilation for x86_64 linux
