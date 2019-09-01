# Nerve

Nerve is a compiler for the esoteric programming language [brainfuck](https://esolangs.org/wiki/Brainfuck).

## Usage

```bash
# compile `hello.b` to `hello.s`
nerve hello.b

# assemble `hello.s` to `hello.o`
nasm hello.s -f elf64 -o hello.o

# link `hello.o` to `hello`
ld hello.o -o hello

# run `hello`
./hello
```

## Running the Nerve tests

Nerve includes an integration test suite which requires `nasm` and `ld` to run. Once you have installed these dependencies, you can run the test suite with `cargo test`.

## Benchmarks

Running `cargo bench` will compile and run the example programs in the `bf-examples` directory. These benchmarks measure the performance of the compiled binaries, not the performance of Nerve itself.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
