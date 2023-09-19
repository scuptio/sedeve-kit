#[cfg(test)]
mod test {
    use bincode::{Decode, Encode};
    use scupt_util::message::{Message, MsgTrait};
    use scupt_util::node_id::NID;
    use serde::{Deserialize, Serialize};
    use crate::action::action_serde_json_string::ActionSerdeJsonString;
    use crate::action::action_message::ActionMessage;
    use crate::action::action_type::ActionType;
    use crate::action::trace::{Trace, TraceJsonValue};



    #[derive(
    Clone,
    Serialize,
    Hash,
    PartialEq,
    Eq,
    Debug,
    Deserialize,
    Decode,
    Encode,
    )]
    struct TestMessage {
        x : i32,
        y : String
    }

    impl MsgTrait for TestMessage {

    }


    fn gen_test_action(
        action_type:ActionType,
        source_node:NID,
        dest_node:NID,
        x:i32,
        y:String
    ) -> ActionMessage<TestMessage> {
        let m = Message::new(
            TestMessage {
                x,
                y
            },
            source_node, dest_node,

        );
        action_type.action_message(m)
    }

    fn gen_trace() -> Trace<TestMessage> {
        let mut trace = Trace::new();
        let a1 = gen_test_action(ActionType::Input, 1, 1, 1, "xxx1".to_string());
        let a2 = gen_test_action(ActionType::Internal, 2, 2, 2, "xxx2".to_string());
        let a3 = gen_test_action(ActionType::Output, 3, 3, 3, "xxx3".to_string());
        let a4 = gen_test_action(ActionType::Input, 4, 4, 4, "xxx4 ".to_string());
        trace.actions.push(a1);
        trace.actions.push(a2);
        trace.actions.push(a3);
        trace.actions.push(a4);
        trace
    }

        const TEST_JSON_STR0:&str = r###"
[
    {"Input":{"source":1,"dest":1,"payload":{"x":1,"y":"xxx1"}}},
    {"Internal":{"source":2,"dest":2,"payload":{"x":2,"y":"xxx2"}}},
    {"Output":{"source":3,"dest":3,"payload":{"x":3,"y":"xxx3"}}},
    {"Input":{"source":4,"dest":4,"payload":{"x":4,"y":"xxx4 "}}}
]
"###;
        const TEST_JSON_STR: &str = r###"
{
    "actions":
    [
        {"Input":{"source":1,"dest":1,"payload":{"x":1,"y":"xxx1"}}},
        {"Internal":{"source":2,"dest":2,"payload":{"x":2,"y":"xxx2"}}},
        {"Output":{"source":3,"dest":3,"payload":{"x":3,"y":"xxx3"}}},
        {"Input":{"source":4,"dest":4,"payload":{"x":4,"y":"xxx4 "}}}
    ]
}
"###;

    #[test]
    fn test() {
        let trace = gen_trace();
        let r = trace.to_json_string();
        assert!(r.is_ok());


        let r = TraceJsonValue::from_json_string(TEST_JSON_STR0.to_string());
        assert!(r.is_ok());

        let r = Trace::<TestMessage>::from_json_string(TEST_JSON_STR.to_string()) ;
        assert!(r.is_ok());

        let r = TraceJsonValue::from_json_string(TEST_JSON_STR.to_string());
        assert!(r.is_ok());

        let trace_json_value = r.unwrap();
        for (_n, v) in trace_json_value.actions.iter().enumerate() {
            let action_json_string = ActionSerdeJsonString::from_json_value(v);
            let r  = action_json_string.to_action_serde_json_value();
            assert!(r.is_ok());
            let action_json_value = r.unwrap();
            let json_value = action_json_value.action_json_value_ref();
            let message :ActionMessage<TestMessage> = serde_json::from_str(v.to_string().as_str()).unwrap();
            assert_eq!(message.source_node_id().unwrap(), action_json_value.source_node_id().unwrap());
            assert_eq!(message.source_node_id().unwrap(), action_json_value.dest_node_id().unwrap());
            println!("{:?}", json_value);
        }
    }
}