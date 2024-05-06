
## TODO

1. We have no longer used **-dump format file** to generate trace file. Parsing .dot file is not efficient. The related source code may need to be deleted from the repo.

2. Easy to use tools that can automatic [generate code stubs](../src/rust_gen)

3. We currently only support the Rust project, and the serialization library is limited to use [serde](https://serde.rs/). We may develop some other language binding and serialization library adapter layer.
Develop [C/C++/Python binding](../binding) for the kit(In progress).

4. The current [trace generation algorithm](generate_trace_algorithm.md) may be not efficent.

5. Example repo to use the kit.
[Example](../example/echo) (In progress)
