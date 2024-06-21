## TODO

1. Easy-to-use tools that can automatically [generate code stubs](../src/rust_gen)

2. The current [trace generation algorithm](generate_trace_algorithm.md) may be not efficent.

3. Polish the documents

4. The current tool depends on a fixed second timeout(seconds_wait_message_timeout)[https://github.com/scuptio/sedeve-kit/blob/b910e95d8882e9701b1a84f18182cc855a559539/src/dtm/dtm_player.rs#L34] to identify inconsistency action flow between the spec and the implementation.
This timeout may be triggered when debugging is in progress, resulting in false-positive reports.
