--------------------------------- MODULE echo ---------------------------------



EXTENDS 
    action, 
    Naturals, 
    Sequences, 
    Integers,
    TLC

CONSTANT MESSAGE
CONSTANT SERVER_NODE_ID
CONSTANT CLIENT_NODE_ID
CONSTANT ENABLE_ACTION
CONSTANT OUT_ACTION_PATH
----


VARIABLE v_state
\* state UNSTARTED -> RUNNING -> STOPPED
VARIABLE v_message
VARIABLE __action__


vars == <<
    v_state,
    v_message,
    __action__
>>

vars_view == <<
    v_state,
    v_message
>>


RUNNING == "RUNNING"
STOPPED == "STOPPED"
ENDED == "ENDED"

SaveAction == 
    OUT_ACTION_PATH /= "" =>
        /\ SaveValue(__action__, OUT_ACTION_PATH)

Init ==
    /\ v_message = [source : CLIENT_NODE_ID, dest : SERVER_NODE_ID, name: {"request"}, payload : MESSAGE] 
            \cup [source: CLIENT_NODE_ID, dest: SERVER_NODE_ID, name: {"stop"}]
    /\ v_state = [i \in SERVER_NODE_ID |-> RUNNING]
    /\ __action__ = InitAction(<<>>, <<>>, ENABLE_ACTION)
    

ServerReceive(m, i, j, enable_action) ==
    /\ v_state[i] = RUNNING
    /\ m.source = j
    /\ m.dest = i 
    /\ m.name = "request"
    /\ LET resp == Message(i, j, "response", m.payload)
           a1 == Action(ActionInput, m)
           a2 == Action(ActionOutput, resp)
       IN /\ v_message' = (v_message \ {m}) \cup {resp}
          /\ SetAction(__action__, <<>>, a1 \o a2, enable_action)
    /\ UNCHANGED <<v_state>>

ServerStop(m, i, j , enable_action) ==
    /\ v_state[i] = RUNNING
    /\ m.source = j
    /\ m.dest = i 
    /\ m.name = "stop"
    /\ v_state' = [v_state EXCEPT ![i] = STOPPED]
    /\ LET a == Action(ActionInput, m)
           \* to enforce create a new state
           dummy == Message(i, j, "dummy", NextID)
       IN /\ v_message' = (v_message \ {m}) \cup {dummy}
          /\ SetAction(__action__, <<>>, a, enable_action)


    
Next == 
   \E i \in SERVER_NODE_ID:
        \/ \E j \in CLIENT_NODE_ID:
            \E m \in v_message:
                \/ ServerStop(m, i, j, ENABLE_ACTION)
                \/ ServerReceive(m, i, j, ENABLE_ACTION)


Spec ==
    /\ Init
    /\ [][Next]_vars

----

     

    
===============================================================================


