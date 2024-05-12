--------------------------------- MODULE message ---------------------------------

Message(
    _from,
    _to,
    _message_name,
    _payload) ==
    [
        source |-> _from,
        dest |-> _to,
        name |-> _message_name,
        payload |-> _payload
    ]

MessageNP(
    _from,
    _to,
    _message_name
    ) ==
    [
        source |-> _from,
        dest |-> _to,
        name |-> _message_name
    ]
        
MessageLocal(
    _local_nid,
    _message_name,
    _payload) ==
    Message(_local_nid, _local_nid, _message_name, _payload)


MessageLocalNP(
    _local_nid,
    _message_name) ==
    MessageNP(_local_nid, _local_nid, _message_name)
    

MessageSet(
    _from_set,
    _to_set,
    _message_name,
    _payload) ==
    [
        source : _from_set,
        dest : _to_set,
        name : {_message_name},
        payload : {_payload}
    ]    
    

WithMessage(_m, _channel) ==
    _channel \cup {_m}

WithoutMessage(_m, _channel) ==
    _channel \ {_m}

WithMessageSet(_m_set, _channel) ==
    _channel \cup _m_set
    
===============================================================================