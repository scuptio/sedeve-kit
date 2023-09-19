
## TODO

1. Choose a distributed algorithm(Raft, for example.) to start our proof of concept(POC).
   The original Raft specification [Ongardie](https://github.com/ongardie/raft.tla) explores too many redundant spaces
   and cannot stop in finite steps.
   We must reduce the state space of the specification to make it stop.
   [Vanlightly](https://github.com/Vanlightly/raft-tlaplus/)'s work can be helpful.

2. Prove the state space generated is sufficient to prove the correctness of all state space with a specific setting;

3. Dump state generated to a formatted output file(for example, a JSON file).

    1) TLA+ toolbox's **-dump format file**[Tools Doc](https://lamport.azurewebsites.net/tla/current-tools.pdf)
       command-line options can be useful;

    2) Or we can overwrite constant operator with Java code to customize the
       behavior [see this example](https://github.com/tlaplus/CommunityModules/commit/8245307786478dfb6de01e2c07107f30ead0b202#diff-f225d5ab283c96c1d8b933ce76d102033dcd8dba8a8c78ecd5dc35ac93e6edd9)

   Approach 1) seems easier to realize.

4. A framework can run actions with deterministic order defined in the file.
   The framework simulates the following actions:

    1) message lost/duplicate/delay/reorder;

    2) user-defined other action, such as file io

   The must be no non-deterministic actions when running testing,
   Such as, reading from a random generator is non-deterministic.

5. Compare with other testing approaches (For example, Lineage-driven Fault Injection).