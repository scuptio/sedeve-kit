--------------------------------- MODULE action ---------------------------------

EXTENDS message, GenID, StateDB, Sequences, SequencesExt, FiniteSets, Naturals


ActionInternal  ==  "T"
ActionInput     ==  "I"
ActionOutput    ==  "O"
ActionSetup     ==  "S"
ActionCheck     ==  "C"


_Action(
    _action_type, 
    _payload
) == 
    [
        t |-> _action_type,
        p |-> _payload
    ]
    
    
Action(
    _action_type, 
    _payload
) == 
    <<
        _Action(_action_type, _payload)
    >>


Actions(
    _action_type,
    _payload_set
) ==
    LET _action_set == [
        t : {_action_type},
        p : _payload_set
    ]
    IN SetToSeq(_action_set)


    
CheckAction(_action, _node_id) ==
    \A i \in DOMAIN _action:
        /\ _action[i].t \in {ActionInternal, ActionInput, ActionOutput}
        /\ _action[i].source \in _node_id
        /\ _action[i].dest \in _node_id
        
        
AppendActionSeq(_var, _action_seq) ==
    _var \o _action_seq


AppendActionSet(_var, _action_set) ==
    _var \o SetToSeq(_action_set)


_NodeVariable(
    _var,
    _id,
    _node_ids
) ==
    [x \in DOMAIN _var |-> _var[x][_id]]  

_NodeChangedVariable(
    _old_var, 
    _new_var,
    _id,
    _node_ids
) ==
    LET changed == {x \in DOMAIN _old_var: _old_var[x][_id] # _new_var[x][_id]}
    IN  [x \in changed |-> _new_var[x][_id]]

__ActionSeqOfNode(
    _handle_node_id(_),
    _node_id,
    _action_type,
    _action_name
) ==
    LET payload == _handle_node_id(_node_id)
        msg == Message(_node_id, _node_id, _action_name, payload)
        action == _Action(_action_type, msg)
    IN action
                
__ActionSeqOfEachNodeHandle(
    _handle_node_id(_),
    _node_ids,
    _action_type,
    _action_name
) ==
    LET f == [id \in _node_ids |-> __ActionSeqOfNode(_handle_node_id, id, _action_type, _action_name)]
        s == {f[id] : id \in DOMAIN f}
    IN SetToSeq(s)


ActionsFromHandle(
    _handle_node_id(_),
    _node_ids,
    _action_type,
    _action_name
) ==
    __ActionSeqOfEachNodeHandle(
        _handle_node_id,
        _node_ids,
        _action_type,
        _action_name)


__ActionSeqOfNodeEx(
    _handle_node_id(_, _),
    _node_id,
    _action_type,
    _action_name,
    _context
) ==
    LET payload == _handle_node_id(_node_id, _context)
        msg == Message(_node_id, _node_id, _action_name, payload)
        action == _Action(_action_type, msg)
    IN action


__ActionSeqOfEachNodeHandleEx(
    _handle_node_id(_, _),
    _node_ids,
    _action_type,
    _action_name,
    _context
) ==
    LET f == [id \in _node_ids |-> 
            __ActionSeqOfNodeEx(
                _handle_node_id,
                id,
                _action_type,
                _action_name,
                _context
                )]

        s == {f[id] : id \in DOMAIN f}
    IN SetToSeq(s)


ActionsFromHandleContext(
    _handle_node_id(_, _),
    _node_ids,
    _action_type,
    _action_name,
    _context
) ==
    __ActionSeqOfEachNodeHandleEx(
        _handle_node_id,
        _node_ids,
        _action_type,
        _action_name,
        _context
      )
        
PrevIdOfAction(
    _action
) ==
    _action.p
    
IdOfAction(
    _action
) ==
    _action.i



ContinuousAction(
    _action, 
    _pc) == 
    /\ "id" \in DOMAIN _pc
    /\ _pc.id = PrevIdOfAction(_action)


InitAction(
    _action_sequence1,
    _action_sequence2,
    _enable
) ==
    IF _enable THEN
        [
            p |-> GetID,
            i |-> NextID,
            s |-> _action_sequence1,
            a |-> _action_sequence2 
        ]
    ELSE
       [
            p |-> GetID,
            i |-> GetID,
            s |-> <<>>,
            a |-> <<>> 
        ]


SetAction(
    _action_variable,
    _action_sequence1,
    _action_sequence2,
    _enable
) ==
    IF _enable THEN
        _action_variable' = [
            p |-> _action_variable.i,
            i |-> NextID,
            s |-> _action_sequence1,
            a |-> _action_sequence2 
        ]
    ELSE
        _action_variable' = [
            p |-> GetID,
            i |-> GetID,
            s |-> <<>>,
            a |-> <<>> 
        ]

    
InitActionT(
    _action_variable,
    _action_sequence1,
    _action_sequence2
) ==
    _action_variable = [
        p |-> NextID,
        i |-> NextID,
        s |-> _action_sequence1,
        a |-> _action_sequence2 
    ]

SetActionT(
    _action_variable,
    _action_sequence1,
    _action_sequence2
) ==
    _action_variable' = [
        p |-> _action_variable.i,
        i |-> NextID,
        s |-> _action_sequence1,
        a |-> _action_sequence2 
    ]


                
InitActionEmpty == {}

===============================================================================