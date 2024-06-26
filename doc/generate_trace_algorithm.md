
## Generate trace from state graph


__S1:__  Read from the input and build a [Directed Graph](https://en.wikipedia.org/wiki/Directed_graph) $G = (V, E)$, in which each vertex in set $V$ of the graph is a valid state of the I/O automata (or the system), and each edge in set $E$ is a state transition from one state to another.
  
  
__S2:__ Find the [Strongly Connected Components (SCC)](https://en.wikipedia.org/wiki/Strongly_connected_component) using [Tarjan's algorithm](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm). The SCC set found would be recorded as $T = \{S_1, S_2, .., S_n\}$, where each $S_i (1 \leq i \leq n)$ is a subgraph of $G$ and an SCC.
  
__S3__ For each SCC $S_i (1 \leq i \leq n) \in T$, replace $S_i$ by a new vertex $\psi(S_i)$, and then finally build a new graph $G'$.
  
__S4__ Perform a [Depth-First Search](https://en.wikipedia.org/wiki/Depth-first_search) on $G'$ and find all paths $P$.
  
__S5__ For each path $\{p_1, p_2, .., p_{i-1}, p_i, p_{i+1}.., p_m\} \in P$, if there exists an $S_j \in T$ such that $p_i = \psi(S_i)$, and there are adjacent edges $p_{i-1}, v_k$ and $v_l, p_{i+1}$, where vertex $v_k$ and $v_j$ are in the vertex set of SCC $S_i$, and vertices $p_{i-1}$, $p_{i+1}$, $v_k$, $v_j$ are in the vertex set $E$ of graph $G$, then construct a [Hamiltonian Path](https://en.wikipedia.org/wiki/Hamiltonian_path) of $S_i$, denoted as $\eta(S_i)= \{h_1, h_2, ... h_m\}$, replace $p_i$ by $\eta(S_i)$, and obtain the trace path $\{p_1, p_2, .., p_{i-1}, h_1, h_2, ... h_m, p_{i+1}.., p_m\}$.
