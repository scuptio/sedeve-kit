
## Mapping the logical model (design-level) to the physical system (code-level).

### Physical System Restriction

The system must exhibit determinism and consistency with the logical model.
A deterministic system generates identical outputs when presented with the same inputs. 
Non-deterministic events, such as message loss or duplication, do not adhere to this property. 
Similarly, timeout and system crash events are also non-deterministic. 
To prevent these non-deterministic events, they can be defined as input actions and read from a predefined stream.

### Multiple level specification

[Multiple level specification](multiple_level_spec.md) is employed to reduce the scale of the state space.
### Use the TLA+ model to generate trace

#### 1. Use the TLA+ model(logical model) to generate the .dot file

$trace(A) = s_0, \pi_1, s_1, \pi_2, s_2, ...,  \pi_r, s_r$


In most cases, we are only interested in the first state, $s_0$, and the sequence of the series of actions, $\pi_1, \pi_2, ..., \pi_r$

Output the sequence $s_0, \pi_1, \pi_2, ..., \pi_r$ to a file for the *deterministic
player [action-player](../src/player/bin/main.rs)*.

We define `__action__` variable in each TLA+ specification and used to dump states for generating test cases.

The *action_type* can be "Input", "Output", or "Internal".
The **action** has a one-to-one mapping to action $\pi_i, 1 \le i \le r$.


An example of `__action__` variable dump likes the following (to search `__action__` in [this dot file](../src/data/toolbox_dump.dot)):

  ```
__action__ = [
    i |-> "bc81f477-09ae-4aa5-9599-839dc815a2a4",   // current state UUID 
    p |-> "f3952e3a-7091-41fd-b0d1-1d8d25d12b45",   // previous state UUID
    a |-> <<                                        // a sequence of message list
        [
            p |-> [
                source |-> A_n1,                    // source node id
                dest |-> A_n3,                      // destination node id
                name |-> "Message1",                // message name
                payload |-> {}                      // message payload
            ],
            t |-> "I",                              // action type, Input(I)/Output(O)/Internal(T)
        ],
        [
            p |-> [
                source |-> A_n1, 
                dest |-> A_n3, 
                name |-> "Message1", 
                payload |-> {}
            ],
            t |-> "T",
        ]
    >>
    ]
  ```

