

## Theory of modeling the system

We can find most of the concepts in Nancy Lynch's Distributed Algorithms.

### I/O automata abstraction

*I/O automata* $A$, which we can call an *automata* for simply, consists five components.

* $sig(A)$, a signature $S$ is a triple consisting of three disjoint sets of actions: the input actions, $in(S)$, the output actions, $out(S)$  and the internal actions, $int(S)$

* $state(A)$, a set of states

* $start(A)$, initial states, a nonempty subset of $state(A)$

* $trans(A)$, a state transition relation; for every state $s \in state(A)$ and every action $\pi$ ,  there is state $s' \in state(A)$, and a transition $(s, \pi, s') \in trans(A)$

* $task(A)$, a task partition


### Trace

A *trace*, $trace(A)$, is a finite sequence,

$trace(A) = s_0, \pi_1, s_1, \pi_2, s_2, ...,  \pi_r, s_r$