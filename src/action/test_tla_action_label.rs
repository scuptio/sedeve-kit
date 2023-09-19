#[cfg(test)]
mod test {
    use bincode::{Decode, Encode};
    use scupt_util::message::MsgTrait;
    use serde::{Deserialize, Serialize};

    use crate::action::action_message::ActionMessage;
    use crate::action::constant;
    use crate::action::tla_actions::{tla_action_var_text_to_json, TLAActionSeq};

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
    struct TestPayload {
        x: i32,
        y: i32,
        opt_z: Option<i32>,
        ok: bool,
    }

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
    enum TestML1 {
        TestM1A(TestML2),
        TestM1B(TestML2),
        TestEmptyPayload
    }

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
    enum TestML2 {
        TestM2A(TestPayload),
        TestM2B(TestPayload),
    }


    impl MsgTrait for TestML1 {}

    impl MsgTrait for TestML2 {}

    impl MsgTrait for TestPayload {}

    #[test]
    fn test_tla_action_label() {
        let r = tla_action_var_text_to_json(
            constant::__TEST_TLA_ACTION_VARIABLE_CONST.to_string(),
            constant::__TEST_NODE_ID_MAP.clone());
        assert!(r.is_ok());
        let json_value = r.unwrap();
        let r = TLAActionSeq::from(json_value);
        assert!(r.is_ok());
        let tla_action_seq = r.unwrap();
        println!("{:?}", &tla_action_seq);
        for action in tla_action_seq.actions {
            let action_json_value = action.to_action_json().unwrap();
            let action_json_string = action_json_value.to_action_message();
            println!("{:?}", action_json_string);
            action_json_value.to_action_message();
            let r = action_json_string.to_string();
            assert!(r.is_ok());
            let json_string = r.unwrap();
            let r = ActionMessage::<TestML1>::from_json_string(json_string.clone());
            assert!(r.is_ok());
            let action_message = r.unwrap();
            println!("{:?}", action_message);
            let r = serde_json::from_str(json_string.as_str());
            assert!(r.is_ok());
            let action_message: ActionMessage<TestML1> = r.unwrap();
            println!("{:?}", action_message);
        }
    }
}