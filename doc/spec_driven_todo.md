
## TODO

1. We have no longer used **-dump format file** to generate trace file. Parsing .dot file is not efficient. The related source code may need to be deleted from the repo.

2. We currently only support the Rust project, and the serialization library is limited to use [serde](https://serde.rs/). We may develop some other language binding and serialization library adapter layer.

3. The current [trace generation algorithm](generate_trace_algorithm.md) may be not efficent.

4. Example repo to use the kit.
