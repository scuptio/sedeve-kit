# How To Start

## Configure TLA+ toolbox

[Configure TLA+ toolbox](configuring_toolbox.md)

## Write TLA+ specification

In the TLA+ specification, we use 
[InitAction](https://github.com/scuptio/tlaplus-specification/blob/main/spec/action.tla#L187) 
and 
[SetAction](https://github.com/scuptio/tlaplus-specification/blob/main/spec/action.tla#L209)
to specify the action of [I/O automata](doc/model_the_system.md) and output the state to sqlite database.


## Run TLA+ model checker and generate state database(a sqlite file).

1. Filling Model checker setting:
   
   Specify constant value and Temporal formula

2. In `Model` -> `TLC Options` Page ->  `Parameters` -> `JVM arguments:` box:

   Specify Java property parameter "tlc2.overrides.TLCOverrides" by filling the text,

``` 
-Dtlc2.overrides.TLCOverrides=tlc2.overrides.TLCOverrides:tlc2.overrides.SedeveTLCOverrides
```

   The "tlc2.overrides.TLCOverrides" is used by thethe [ComunityModeules](https://github.com/tlaplus/CommunityModules); and "the tlc2.overrides.SedeveTLCOverrides" is used by [SedeveModules](https://github.com/scuptio/SedeveModules/).

   More about overrides, see [SpecProcessor.java](https://github.com/tlaplus/tlaplus/blob/72793369891dd72d31d6d48444df18b33434c966/tlatools/org.lamport.tlatools/src/tlc2/tool/impl/SpecProcessor.java#L637),
   [TLCOverrides.java](https://github.com/tlaplus/CommunityModules/blob/9c13c116cbcb4975c7d33ee7a71ba471b88687b2/modules/tlc2/overrides/TLCOverrides.java#L29)

3. Click then `Runs TLC on the model` Button
 

## Development by specification


### Generate a trace from the sqlite state database that was output by the TLA+ toolbox.


A trace is a finite sequence of actions. An action is a step of state transition. We define several action types based on their functionalities.
The action types include:

Use the [sedeve_trace-gen](../src/trace_gen/main.rs) to traversal state space
and dump all trace into a database(e.g., [sqlite DB file](../src/data/trace.db))
The trace format is represented in JSON, similar to [this JSON file](../src/data/trace2.json)
The [action incoming interface](../src/dtm/action_incoming.rs) can be used to read action steps of trace.


### Insert *anchor action*s to the testing source code

We define certain *anchor actions* that allow us to send a message to the *deterministic player* for reordering the actions.

### Implement the Rust code

### Add assert invariants to the testing source code

During testing, we add invariants to assert the correctness of our assumptions.

### Running deterministic testing

## Example
[Two-Phase Commit Protocol(2PC)](https://en.wikipedia.org/wiki/Two-phase_commit_protocol) is a atomic commit protocol.
The example of using this kit to develop 2PC could be found at [example-2pc](https://github.com/scuptio/example-2pc).

[Raft](https://raft.github.io/) consensus algorithm.
We implement [scupt-raft](https://github.com/scuptio/scupt-raft)
