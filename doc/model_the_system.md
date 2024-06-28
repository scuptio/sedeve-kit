

## Theory of modeling the system

We can find most of the concepts in Nancy Lynch's Distributed Algorithms.

### I/O automata abstraction

*I/O automata* $A$, which we can call an *automata* for simply, consists five components.

* $sig(A)$, a signature $S$ is a triple consisting of three disjoint sets of actions: 
the input actions, $in(S)$, the output actions, $out(S)$  and the internal actions, $int(S)$

  
| Action Type	  | Description of Action Type 	                                                                    |
|---------------|-------------------------------------------------------------------------------------------------|
| Input | Represent a node receiving an input message, from a network endpoint or a terminal, for example |
| Output        | Represent a node sending an output message, to a network endpoint or a terminal, for example    |
| Internal      | Represent an internal event in a node                                                           |


* $state(A)$, a set of states

* $start(A)$, initial states, a nonempty subset of $state(A)$

* $trans(A)$, a state transition relation; for every state $s \in state(A)$ and every action $\pi$ ,  
there is state $s' \in state(A)$, and a transition $(s, \pi, s') \in trans(A)$

* $task(A)$, a task partition


### Trace

A *trace* of $A$,  $trace(A)$, is a finite sequence,

$trace(A) = s_0, \pi_1, s_1, \pi_2, s_2, ...,  \pi_r, s_r$

in which, there are  $s_i \in state(A), 0 \le i \le r$ and
$(s_i, \pi_{i+1}, s_{i+1}) \in trans(A), 0 \le i < r$, $s_0$ is ${start(A)}$.

The state transition graph of an I/O automata produces a collection of *trace* sets, 
which are used to generate a set of test cases. 
